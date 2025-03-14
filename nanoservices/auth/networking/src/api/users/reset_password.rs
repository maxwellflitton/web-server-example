//! Networking layer for resetting a users password
use dal::users::tx_definitions::ResetPassword;
use auth_core::api::users::reset_password::reset_password as reset_password_core;
use actix_web::{
    HttpResponse,
    web::Json
};
use serde::Deserialize;
use utils::api_endpoint;


/// Schema for resetting the password for a user
/// 
/// # Fields
/// * `uuid` - The unqiue ID of the user.
/// * `new_password` - The users new password.
#[derive(Deserialize)]
pub struct ResetPasswordSchema {
    pub unique_id: String,
    pub new_password: String,
}

#[api_endpoint(db_traits=[ResetPassword])]
pub async fn reset_password(body: Json<ResetPasswordSchema>) {
    let _ = reset_password_core::<X>(&body.unique_id, &body.new_password).await?;
    Ok(HttpResponse::Ok().finish())
}


#[cfg(test)]
mod tests {
    //! Tests for the `reset_password` HTTP endpoint.
    //!
    //! These tests validate the behavior of the `reset_password` function by simulating a
    //! successful confirmation using a mock database implementation.

    use super::*;
    use actix_web::{
        dev::ServiceResponse,
        http::header::ContentType,
        test::{call_service, init_service, TestRequest},
        web, App,
    };
    use actix_http::Request;
    use dal_tx_impl::impl_transaction;
    use serde_json::json;
    use utils::errors::NanoServiceError;

    #[tokio::test]
    async fn test_reset_password_success() {
        // Define our mock database handle.
        struct MockDbHandle;

        // Provide a mock implementation for the `ResetPassword` transaction.
        #[impl_transaction(MockDbHandle, ResetPassword, reset_password)]
        async fn reset_password(uuid: String, _new_password: String) -> Result<bool, NanoServiceError> {
            // Ensure that the `unique_id` received matches our expectation.
            assert_eq!(uuid, "unique-123");
            Ok(true)
        }

        // Helper function to run our test request.
        async fn run_request(req: Request) -> ServiceResponse {
            // Instantiate the endpoint with our mock type.
            let service = reset_password::<MockDbHandle>;
            let app = init_service(App::new().route("/reset_password", web::post().to(service))).await;
            call_service(&app, req).await
        }

        // Build the JSON body expected by the endpoint.
        let body = json!({
            "unique_id": "unique-123",
            "new_password": "new_password"
        });

        // Construct the test request.
        let req = TestRequest::post()
            .insert_header(ContentType::json())
            .uri("/reset_password")
            .set_json(&body)
            .to_request();

        // Execute the request and verify the response.
        let resp = run_request(req).await;
        assert_eq!(resp.status(), 200);
    }
}
