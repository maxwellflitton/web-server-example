//! Networking layer for confirming a user
use dal::users::tx_definitions::ConfirmUser;
use auth_core::api::users::confirm_user::confirm_user as confirm_user_core;
use actix_web::{
    HttpResponse,
    web::Json
};
use serde::Deserialize;
use utils::api_endpoint;


/// Schema for confirming a user
/// 
/// # Fields
/// * `unique_id` - The unqiue ID of the user to confirm.
#[derive(Deserialize)]
pub struct ConfirmUserSchema {
    pub unique_id: String
}

#[api_endpoint(db_traits=[ConfirmUser])]
pub async fn confirm_user(body: Json<ConfirmUserSchema>) {
    let _ = confirm_user_core::<X>(&body.unique_id).await?;
    Ok(HttpResponse::Ok().finish())
}


#[cfg(test)]
mod tests {
    //! Tests for the `confirm_user` HTTP endpoint.
    //!
    //! These tests validate the behavior of the `confirm_user` function by simulating a
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
    async fn test_confirm_user_success() {
        // Define our mock database handle.
        struct MockDbHandle;

        // Provide a mock implementation for the `ConfirmUser` transaction.
        #[impl_transaction(MockDbHandle, ConfirmUser, confirm_user)]
        async fn confirm_user(unique_id: String) -> Result<bool, NanoServiceError> {
            // Ensure that the `unique_id` received matches our expectation.
            assert_eq!(unique_id, "unique-123");
            Ok(true)
        }

        // Helper function to run our test request.
        async fn run_request(req: Request) -> ServiceResponse {
            // Instantiate the endpoint with our mock type.
            let service = confirm_user::<MockDbHandle>;
            let app = init_service(App::new().route("/confirm_user", web::post().to(service))).await;
            call_service(&app, req).await
        }

        // Build the JSON body expected by the endpoint.
        let body = json!({
            "unique_id": "unique-123",
        });

        // Construct the test request.
        let req = TestRequest::post()
            .insert_header(ContentType::json())
            .uri("/confirm_user")
            .set_json(&body)
            .to_request();

        // Execute the request and verify the response.
        let resp = run_request(req).await;
        assert_eq!(resp.status(), 200);
    }
}
