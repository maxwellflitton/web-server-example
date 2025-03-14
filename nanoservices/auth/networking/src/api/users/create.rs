//! HTTP endpoint for creating a user.
//!
//! # Overview
//! This file defines the HTTP endpoint for creating a user using Actix Web. It serves as the 
//! networking layer that wraps the core functionality of user creation and exposes it via a RESTful API.
//!
//! # Features
//! - Accepts JSON input representing the user to be created.
//! - Delegates core logic to the `create_user` function in the core logic workspace.
//! - Returns appropriate HTTP responses based on the outcome of the operation.
//!
//! # Notes
//! - The function is generic and allows different database implementations to be injected.
//! - Additional actions, such as sending an email, can be performed after the user is created.
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
use kernel::users::NewUserSchema;
use auth_core::api::users::create::create_user as create_user_core;
use actix_web::{
    HttpResponse,
    web::Json
};
use utils::api_endpoint;


/// This is our networking method for creating a user
///
/// # Notes
/// - We call our core method with the traits in the order <X, W, Y> because the core method takes the db traits struct, then the 
///   email traits struct, then lastly the env variable trait struct. 
/// - The way our `api_endpoint` macro defines the traits is W for the email traits, X for the db traits and Y for the env variable
///   trait.
#[api_endpoint(
    token=SuperAdminRoleCheck, 
    db_traits=[CreateUser, CreateRolePermission, CreateRateLimitEntry, UpdateRateLimitEntry, GetRateLimitEntry], 
    email_traits=[SendTemplate])
]
pub async fn create_user(body: Json<NewUserSchema>) {
    let _ = create_user_core::<X, W, Y>(body.into_inner()).await?;
    Ok(HttpResponse::Created().finish())
}


#[cfg(test)]
mod tests {
    //! Tests for the `create` HTTP endpoint.
    //!
    //! These tests validate the behavior of the `create` function with both successful and error outcomes,
    //! using a mock database implementation.

    use super::*;
    use actix_web::http::header;
    use actix_web::{
        dev::ServiceResponse,
        self, body::MessageBody, http::header::ContentType, test::{
            call_service, init_service, TestRequest
        }, web, App
    };
    use actix_http::Request;
    use kernel::users::{User, NewUser};
    use kernel::rate_limit_entries::{RateLimitEntry, NewRateLimitEntry};
    use dal_tx_impl::impl_transaction;
    use kernel::role_permissions::{RolePermission, NewRolePermission};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::LazyLock;
    use kernel::users::UserRole;
    use serde_json::json;
    use utils::errors::NanoServiceError;
    use kernel::token::token::HeaderToken;
    use kernel::token::session_cache::engine_mock::PassAuthSessionCheckMock;
    use utils::config::GetConfigVariable;
    use email_core::mailchimp_helpers::mailchimp_template::Template;
    use kernel::token::checks::SuperAdminRoleCheck;
    use chrono::{Utc, Duration};

    fn generate_user(user: NewUser) -> User {
        let now = chrono::Utc::now().naive_utc();
        User {
            id: 1,
            confirmed: false,
            username: user.username.clone(),
            email: user.email.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            user_role: user.user_role.clone(),
            password: user.password.clone(),
            uuid: user.uuid.clone(),
            date_created: now,
            last_logged_in: now,
            blocked: user.blocked,
        }
    }

    #[tokio::test]
    async fn test_pass() {

        static SEND_TEMPLATE_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
        static CREATE_USER_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
        static CREATE_ROLE_PERMISSION_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
       
        struct MockDbHandle;
        struct MockMailchimpHandle;
        struct MockConfig;
        
        #[impl_transaction(MockDbHandle, CreateUser, create_user)]
        async fn create_user(user: NewUser) -> Result<User, NanoServiceError> {
            CREATE_USER_CALLED.store(true, Ordering::Relaxed);
            Ok(generate_user(user))
        }

        #[impl_transaction(MockDbHandle, CreateRolePermission, create_role_permission)]
        async fn create_role_permission(role_permission: NewRolePermission) -> Result<RolePermission, NanoServiceError> {
            CREATE_ROLE_PERMISSION_CALLED.store(true, Ordering::Relaxed);
            Ok(RolePermission{
                id: 1,
                user_id: role_permission.user_id,
                role: role_permission.role.clone()
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

        #[impl_transaction(MockMailchimpHandle, SendTemplate, send_template)]
        async fn send_template(_template: &Template) -> Result<bool, NanoServiceError> {
            SEND_TEMPLATE_CALLED.store(true, Ordering::Relaxed);
            Ok(true)
        }

        impl GetConfigVariable for MockConfig {
            fn get_config_variable(variable: String) -> Result<String, NanoServiceError> {
                match variable.as_str() {
                    "MAILCHIMP_API_KEY" => Ok("mock_mailchimp_api".to_string()),
                    "PRODUCTION" => Ok("true".to_string()),
                    _ => Ok("".to_string()),
                }
            }
        }

        async fn run_request(req: Request) -> ServiceResponse {
            let service = create_user::<MockMailchimpHandle, MockDbHandle, MockConfig, PassAuthSessionCheckMock>;
            let app = init_service(App::new().route("/create", web::post().to(service))).await;
            call_service(&app, req).await
        }

        // password is not needed as we are autogenerating it
        let body = json!({
            "email": "zak@gmail.com",
            "username": "admin_user",
            "password": "password",
            "first_name": "zak",
            "last_name": "siddiq",
            "user_role": "AdMiN",
        });

        let agent = "some-agent".to_string();

        let jwt: HeaderToken<MockConfig, SuperAdminRoleCheck> = HeaderToken::new(
            agent.clone(), 
            1, 
            UserRole::SuperAdmin,
        );

        let req = TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("token", jwt.encode().unwrap()))
            .insert_header((header::USER_AGENT, agent))
            .uri("/create")
            .set_json(&body)
            .to_request();
        let resp = run_request(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let _body_str = std::str::from_utf8(&raw_body).unwrap();

        assert!(CREATE_USER_CALLED.load(Ordering::Relaxed));
        assert!(SEND_TEMPLATE_CALLED.load(Ordering::Relaxed));
        assert!(CREATE_ROLE_PERMISSION_CALLED.load(Ordering::Relaxed));

        assert_eq!(status, 201);
    }

    #[tokio::test]
    async fn test_bad_json() {

        static SEND_TEMPLATE_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
        static CREATE_USER_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
        static CREATE_ROLE_PERMISSION_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
       
        struct MockDbHandle;
        struct MockMailchimpHandle;
        struct MockConfig;
        
        #[impl_transaction(MockDbHandle, CreateUser, create_user)]
        async fn create_user(user: NewUser) -> Result<User, NanoServiceError> {
            CREATE_USER_CALLED.store(true, Ordering::Relaxed);
            Ok(generate_user(user))
        }

        #[impl_transaction(MockDbHandle, CreateRolePermission, create_role_permission)]
        async fn create_role_permission(role_permission: NewRolePermission) -> Result<RolePermission, NanoServiceError> {
            CREATE_ROLE_PERMISSION_CALLED.store(true, Ordering::Relaxed);
            Ok(RolePermission{
                id: 1,
                user_id: role_permission.user_id,
                role: role_permission.role.clone()
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

        #[impl_transaction(MockMailchimpHandle, SendTemplate, send_template)]
        async fn send_template(_template: &Template) -> Result<bool, NanoServiceError> {
            SEND_TEMPLATE_CALLED.store(true, Ordering::Relaxed);
            Ok(true)
        }

        impl GetConfigVariable for MockConfig {
            fn get_config_variable(variable: String) -> Result<String, NanoServiceError> {
                match variable.as_str() {
                    "MAILCHIMP_API_KEY" => Ok("mock_mailchimp_api".to_string()),
                    "PRODUCTION" => Ok("true".to_string()),
                    _ => Ok("".to_string()),
                }
            }
        }
        async fn run_request(req: Request) -> ServiceResponse {
            let service = create_user::<MockMailchimpHandle, MockDbHandle, MockConfig, PassAuthSessionCheckMock>;
            let app = init_service(App::new().route("/create", web::post().to(service))).await;
            call_service(&app, req).await
        }

        let body = json!({
            "email": "zak@gmail.com",
            "username": "admin_user",
            // "password": "password",
            // "first_name": "zak",
            // "last_name": "siddiq",
            "user_role": "AdMiN",
            "password": "securepassword"
        });

        let agent = "some-agent".to_string();

        let jwt: HeaderToken<MockConfig, SuperAdminRoleCheck> = HeaderToken::new(
            agent.clone(), 
            1, 
            UserRole::SuperAdmin,
        );

        let req = TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("token", jwt.encode().unwrap()))
            .insert_header((header::USER_AGENT, agent))
            .uri("/create")
            .set_json(&body)
            .to_request();
        let resp = run_request(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let _body_str = std::str::from_utf8(&raw_body).unwrap();


        assert!(!SEND_TEMPLATE_CALLED.load(Ordering::Relaxed));
        assert!(!CREATE_USER_CALLED.load(Ordering::Relaxed));
        assert!(!CREATE_ROLE_PERMISSION_CALLED.load(Ordering::Relaxed));

        assert_eq!(status, 400);
    }

}
