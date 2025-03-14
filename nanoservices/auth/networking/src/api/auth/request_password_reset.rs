//! HTTP endpoint for requesting a password reset email.
//!
//! # Overview
//! This file defines the HTTP endpoint for requesting a password reset email using Actix Web. It serves as the 
//! networking layer that wraps the core functionality of user creation and exposes it via a RESTful API.
//!
//! # Features
//! - Accepts JSON input representing the user to be created.
//! - Delegates core logic to the `reset_password_request` function in the core logic workspace.
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
// use dal::users::tx_definitions::CreateUser;
// use dal::role_permissions::tx_definitions::CreateRolePermission;
// use kernel::users::NewUserSchema;
use auth_core::api::auth::request_password_reset::request_password_reset as request_password_reset_core;
use actix_web::{
    HttpResponse,
    web::Json
};
use utils::api_endpoint;
use dal::users::tx_definitions::UpdateUuid;
use dal::rate_limit_entries::tx_definitions::{
    CreateRateLimitEntry,
    UpdateRateLimitEntry,
    GetRateLimitEntry,
};
use email_core::mailchimp_traits::mc_definitions::SendTemplate;
use serde::Deserialize;


/// Schema for requesting a users password reset
/// 
/// # Fields
/// * `email` - The email of the user.
#[derive(Deserialize)]
pub struct RequestPasswordResetSchema {
    pub email: String
}


/// This is our networking method for requesting a password reset
///
/// # Notes
/// - We call our core method with the traits in the order <X, W, Y> because the core method takes the db traits struct, then the 
///   email traits struct, then lastly the env variable trait struct. 
/// - The way our `api_endpoint` macro defines the traits is W for the email traits, X for the db traits and Y for the env variable
///   trait.
#[api_endpoint(db_traits=[CreateRateLimitEntry, UpdateRateLimitEntry, GetRateLimitEntry, UpdateUuid], email_traits=[SendTemplate], env_variable_trait=true)]
pub async fn request_password_reset(body: Json<RequestPasswordResetSchema>) {
    let body = body.into_inner();
    let _ = request_password_reset_core::<X, W, Y>(body.email.clone()).await?;
    Ok(HttpResponse::Ok().finish())
}


#[cfg(test)]
mod tests {
    //! Tests for the `request_password_reset` HTTP endpoint.
    //!
    //! These tests validate the behavior of the `request_password_reset` function with both
    //! successful and error outcomes, using mock implementations of the database and email services,
    //! closely mirroring how the `create` endpoint is tested.

    use super::*;
    use actix_web::{
        body::MessageBody,
        dev::ServiceResponse,
        http::header::ContentType,
        test::{call_service, init_service, TestRequest},
        web, App,
    };
    use actix_http::Request;
    use dal_tx_impl::impl_transaction;
    use email_core::mailchimp_traits::mc_definitions::SendTemplate;
    use email_core::mailchimp_helpers::mailchimp_template::Template;
    use dal::rate_limit_entries::tx_definitions::{
        CreateRateLimitEntry, UpdateRateLimitEntry, GetRateLimitEntry,
    };
    use dal::users::tx_definitions::UpdateUuid;
    use kernel::rate_limit_entries::{NewRateLimitEntry, RateLimitEntry};
    use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
    use utils::config::GetConfigVariable;
    use chrono::{Duration, Utc};
    use serde_json::json;

    // -- Mock Implementations --

    // 1) Mock "success" DB handle
    struct MockDbHandleSuccess;

    #[impl_transaction(MockDbHandleSuccess, UpdateUuid, update_uuid)]
    async fn update_uuid(email: String, _new_uuid: String) -> Result<bool, NanoServiceError> {
        match email.as_str() {
            "example@gmail.com" => Ok(true),
            "returnfalse@gmail.com" => Ok(false),
            _ => Err(NanoServiceError::new(
                "Error updating user".to_string(),
                NanoServiceErrorStatus::NotFound,
            )),
        }
    }

    #[impl_transaction(MockDbHandleSuccess, CreateRateLimitEntry, create_rate_limit_entry)]
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

    #[impl_transaction(MockDbHandleSuccess, GetRateLimitEntry, get_rate_limit_entry)]
    async fn get_rate_limit_entry(email: String) -> Result<Option<RateLimitEntry>, NanoServiceError> {
        Ok(Some(RateLimitEntry {
            id: 1,
            email,
            rate_limit_period_start: Utc::now().naive_utc() - Duration::hours(2),
            count: 2,
        }))
    }

    #[impl_transaction(MockDbHandleSuccess, UpdateRateLimitEntry, update_rate_limit_entry)]
    async fn update_rate_limit_entry(
        _updated_entry: RateLimitEntry,
    ) -> Result<bool, NanoServiceError> {
        Ok(true)
    }

    // 2) Mock Mailchimp "ok"
    struct MockMailchimpHandleOk;

    #[impl_transaction(MockMailchimpHandleOk, SendTemplate, send_template)]
    async fn send_template(_template: &Template) -> Result<bool, NanoServiceError> {
        Ok(true)
    }

    // 3) Mock Mailchimp "return false"
    struct MockMailchimpHandleReturnFalse;

    #[impl_transaction(MockMailchimpHandleReturnFalse, SendTemplate, send_template)]
    async fn send_template(_template: &Template) -> Result<bool, NanoServiceError> {
        Ok(false)
    }

    // 4) Mock Mailchimp "error"
    struct MockMailchimpHandleError;

    #[impl_transaction(MockMailchimpHandleError, SendTemplate, send_template)]
    async fn send_template(_template: &Template) -> Result<bool, NanoServiceError> {
        Err(NanoServiceError::new(
            "Error sending email template".to_string(),
            NanoServiceErrorStatus::Unknown,
        ))
    }

    // 5) Fake config
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

    // Helper function to run our test request with correct traits
    async fn run_request_success(req: Request) -> ServiceResponse {
        let service = request_password_reset::<MockMailchimpHandleOk, MockDbHandleSuccess, FakeConfig>;
        let app = init_service(App::new().route("/request_password_reset", web::post().to(service))).await;
        call_service(&app, req).await
    }

    // Helper function to run our test request with a mailchimp trait which returns false
    async fn run_request_mailchimp_return_false(req: Request) -> ServiceResponse {
        let service = request_password_reset::<MockMailchimpHandleReturnFalse, MockDbHandleSuccess, FakeConfig>;
        let app = init_service(App::new().route("/request_password_reset", web::post().to(service))).await;
        call_service(&app, req).await
    }

    // Helper function to run our test request with a mailchimp trait which returns an error
    async fn run_request_mailchimp_return_error(req: Request) -> ServiceResponse {
        let service = request_password_reset::<MockMailchimpHandleError, MockDbHandleSuccess, FakeConfig>;
        let app = init_service(App::new().route("/request_password_reset", web::post().to(service))).await;
        call_service(&app, req).await
    }

    /// 1) Test Success
    #[tokio::test]
    async fn test_success() {
        let body = json!({ "email": "example@gmail.com" });

        let req = TestRequest::post()
            .uri("/request_password_reset")
            .insert_header(ContentType::json())
            .set_json(&body)
            .to_request();

        let resp = run_request_success(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let _body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(status, 200, "Should return 200 on success");
    }

    /// 2) Test update_uuid return false
    #[tokio::test]
    async fn test_update_uuid_return_false() {
        let body = json!({ "email": "returnfalse@gmail.com" });

        let req = TestRequest::post()
            .uri("/request_password_reset")
            .insert_header(ContentType::json())
            .set_json(&body)
            .to_request();

        let resp = run_request_success(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(status, 500, "Should return 500 on update_uuid return false");
        assert!(body_str.contains("Failed to update users uuid"));
    }

    /// 3) Test update_uuid error
    #[tokio::test]
    async fn test_update_uuid_error() {
        let body = json!({ "email": "wrongemail@gmail.com" });

        let req = TestRequest::post()
            .uri("/request_password_reset")
            .insert_header(ContentType::json())
            .set_json(&body)
            .to_request();

        let resp = run_request_success(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(status, 404, "Should return 404 on update_uuid error (NotFound)");
        assert!(body_str.contains("Error updating user"));
    }

    /// 4) Test send_template return false
    #[tokio::test]
    async fn test_send_template_return_false() {
        let body = json!({ "email": "example@gmail.com" });

        let req = TestRequest::post()
            .uri("/request_password_reset")
            .insert_header(ContentType::json())
            .set_json(&body)
            .to_request();

        let resp = run_request_mailchimp_return_false(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        // If send_template returns false, we get an Unknown error -> 500
        assert_eq!(status, 500, "Should return 500 on send_template returning false");
        assert!(body_str.contains("Failed to send password reset email due to a rate limit error"));
    }

    /// 5) Test send_template error
    #[tokio::test]
    async fn test_send_template_error() {
        let body = json!({ "email": "example@gmail.com" });

        let req = TestRequest::post()
            .uri("/request_password_reset")
            .insert_header(ContentType::json())
            .set_json(&body)
            .to_request();

        let resp = run_request_mailchimp_return_error(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        // If send_template errors, we get an Unknown -> 500
        assert_eq!(status, 500, "Should return 500 on send_template error");
        assert!(body_str.contains("Error sending email template"));
    }

    /// 6) Test with invalid JSON (bad body) – typical 400 scenario
    ///    e.g. The endpoint expects {"email": ...}, let's omit it or pass something invalid.
    #[tokio::test]
    async fn test_bad_json() {
        // We'll exclude `email` to simulate invalid input
        let body = json!({ "username": "missing_email" });

        let req = TestRequest::post()
            .uri("/request_password_reset")
            .insert_header(ContentType::json())
            .set_json(&body)
            .to_request();

        let resp = run_request_success(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        // Usually this yields a 400, depending on how Actix parses and your JSON schema
        assert_eq!(status, 400, "Should return 400 for missing required email field");
        assert!(body_str.contains("missing field") || body_str.contains("bad request"));
    }
}
