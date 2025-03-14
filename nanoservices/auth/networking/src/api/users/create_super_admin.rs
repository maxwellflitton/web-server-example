//! HTTP handler for creating a user.
//!
//! # Arguments
//! - `body`: A JSON representation of `NewUserSchema` containing the user's details.
//!
//! # Returns
//! - `Ok(HttpResponse)`: A 201 Created response if the user is successfully created.
//! - `Err(NanoServiceError)`: A 500 Internal Server Error response if the operation fails.
//!
//! # Notes
//! - After delegating to the core `create_user` function, additional actions (e.g., sending an email) can be performed.
//! - This function uses generics to allow the injection of different implementations of the `CreateUser` trait.
use dal::users::tx_definitions::CreateUser;
use dal::rate_limit_entries::tx_definitions::{
    CreateRateLimitEntry,
    UpdateRateLimitEntry,
    GetRateLimitEntry,
};
use email_core::mailchimp_traits::mc_definitions::SendTemplate;
use dal::role_permissions::tx_definitions::CreateRolePermission;
use auth_core::api::users::create_super_admin::create_super_user as create_super_user_core;
use actix_web::{web::Json, HttpResponse};
use serde::Deserialize;
use kernel::users::UserRole;
use utils::api_endpoint;


#[derive(Deserialize)]
pub struct SuperAdminSchema {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub user_role: UserRole,
    pub password: String
}


/// This is our networking method for creating a super admin
///
/// # Notes
/// - We call our core method with the traits in the order <X, W, Y> because the core method takes the db traits struct, then the 
///   email traits struct, then lastly the env variable trait struct. 
/// - The way our `api_endpoint` macro defines the traits is W for the email traits, X for the db traits and Y for the env variable
///   trait.
#[api_endpoint(db_traits=[CreateUser, CreateRolePermission, CreateRateLimitEntry, UpdateRateLimitEntry, GetRateLimitEntry], email_traits=[SendTemplate], env_variable_trait=true)]
pub async fn create_super_user(body: Json<SuperAdminSchema>) {
    let body = body.into_inner();
    let _ = create_super_user_core::<X, W, Y>(
        body.username,
        body.email,
        body.first_name,
        body.last_name,
        body.password
    ).await?;
    Ok(HttpResponse::Created().json({}))
}


#[cfg(test)]
mod tests {
    //! Tests for the `create` HTTP endpoint.
    //!
    //! These tests validate the behavior of the `create` function with both successful and error outcomes,
    //! using a mock database implementation.

    use super::*;
    use actix_web::{
        dev::ServiceResponse,
        self, body::MessageBody, http::header::ContentType, test::{
            call_service, init_service, TestRequest
        }, web, App
    };
    use kernel::role_permissions::{RolePermission, NewRolePermission};
    use kernel::rate_limit_entries::{RateLimitEntry, NewRateLimitEntry};
    use actix_http::Request;
    use kernel::users::{User, NewUser};
    use dal_tx_impl::impl_transaction;
    use serde_json::json;
    use utils::errors::NanoServiceError;
    use chrono::{Utc, Duration};
    use utils::config::GetConfigVariable;
    use email_core::mailchimp_helpers::mailchimp_template::Template;

    struct MockDbHandle;

    #[impl_transaction(MockDbHandle, CreateUser, create_user)]
    async fn create_user(user: NewUser) -> Result<User, NanoServiceError> {
        Ok(User {
            id: 1, // Mock ID
            confirmed: false,
            username: user.username.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            user_role: user.user_role.clone(),
            date_created: Utc::now().naive_utc(),
            last_logged_in: Utc::now().naive_utc(),
            blocked: user.blocked,
            uuid: user.uuid.clone(),
        })
    }

    #[impl_transaction(MockDbHandle, CreateRolePermission, create_role_permission)]
    async fn create_role_permission(role_permission: NewRolePermission) -> Result<RolePermission, NanoServiceError> {
        Ok(RolePermission {
            id: 1, // Mock ID
            user_id: role_permission.user_id,
            role: role_permission.role.clone(),
        })
    }

    #[impl_transaction(MockDbHandle, CreateRateLimitEntry, create_rate_limit_entry)]
    async fn create_rate_limit_entry(
        new_entry: NewRateLimitEntry,
    ) -> Result<RateLimitEntry, NanoServiceError> {
        Ok(RateLimitEntry {
            id: 1,
            email: new_entry.email.clone(),
            rate_limit_period_start: Utc::now().naive_utc(),
            count: 1,
        })
    }

    #[impl_transaction(MockDbHandle, GetRateLimitEntry, get_rate_limit_entry)]
    async fn get_rate_limit_entry(email: String) -> Result<Option<RateLimitEntry>, NanoServiceError> {
        Ok(Some(RateLimitEntry {
            id: 1,
            email,
            rate_limit_period_start: Utc::now().naive_utc() - Duration::hours(2),
            count: 2,
        }))
    }

    #[impl_transaction(MockDbHandle, UpdateRateLimitEntry, update_rate_limit_entry)]
    async fn update_rate_limit_entry(
        _updated_entry: RateLimitEntry,
    ) -> Result<bool, NanoServiceError> {
        Ok(true)
    }

    struct MockMailchimpHandle;

    #[impl_transaction(MockMailchimpHandle, SendTemplate, send_template)]
    async fn send_template(_template: &Template) -> Result<bool, NanoServiceError> {
        Ok(true)
    }

    struct FakeConfig;

    impl GetConfigVariable for FakeConfig {
        fn get_config_variable(variable: String) -> Result<String, NanoServiceError> {
            match variable.as_str() {
                "MAILCHIMP_API_KEY" => Ok("mock_mailchimp_api".to_string()),
                "PRODUCTION" => Ok("true".to_string()),
                _ => Ok("".to_string()),
            }
        }
    }

    async fn run_request(req: Request) -> ServiceResponse {
        let service = create_super_user::<MockMailchimpHandle, MockDbHandle, FakeConfig>;
        let app = init_service(App::new().route("/create", web::post().to(service))).await;
        call_service(&app, req).await
    }

    #[tokio::test]
    async fn test_create_super_user_ok() {
        let body = json!({
            "email": "zak@gmail.com",
            "username": "superadmin_user",
            "password": "password",
            "first_name": "zak",
            "last_name": "siddiq",
            "user_role": "SuPeR AdMiN",
            "password": "securepassword"
        });
        let req = TestRequest::post()
            .insert_header(ContentType::json())
            .uri("/create")
            .set_json(&body)
            .to_request();
        let resp = run_request(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(status, 201);
        assert_eq!(body_str, "null");
    }

    #[tokio::test]
    async fn test_create_super_user_wrong_email() {
        let body = json!({
            "email": "zakk@gmail.com",
            "username": "superadmin_user",
            "password": "password",
            "first_name": "zak",
            "last_name": "siddiq",
            "user_role": "SuPeR AdMiN",
            "password": "securepassword"
        });
        let req = TestRequest::post()
            .insert_header(ContentType::json())
            .uri("/create")
            .set_json(&body)
            .to_request();
        let resp = run_request(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(status, 401);
        assert_eq!(body_str, "\"email: zakk@gmail.com is not allowed to be a super admin\"");
    }

    #[tokio::test]
    async fn test_create_super_user_wrong_user_role() {
        let body = json!({
            "email": "zakk@gmail.com",
            "username": "superadmin_user",
            "password": "password",
            "first_name": "zak",
            "last_name": "siddiq",
            "user_role": "SuPeR AdMiNsssss"
        });
        let req = TestRequest::post()
            .insert_header(ContentType::json())
            .uri("/create")
            .set_json(&body)
            .to_request();
        let resp = run_request(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(status, 400);
        assert_eq!(body_str, "Json deserialize error: Invalid user role: SuPeR AdMiNsssss at line 1 column 118");
    }

}
