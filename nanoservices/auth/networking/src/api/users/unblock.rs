//! Networking layer for unblocking a user
use dal::users::tx_definitions::UnblockUser;
use auth_core::api::users::unblock::unblock_user as unblock_user_core;
use actix_web::{
    HttpResponse,
    web::Json
};
use serde::Deserialize;
use utils::api_endpoint;


/// Schema for unblocking a user
/// 
/// # Fields
/// * `user_id` - The ID of the user to unblock.
#[derive(Deserialize)]
pub struct UnblockSchema {
    pub user_id: i32
}


#[api_endpoint(token=SuperAdminRoleCheck, db_traits=[UnblockUser])]
pub async fn unblock_user(body: Json<UnblockSchema>) {
    let _ = unblock_user_core::<X>(body.user_id).await?;
    Ok(HttpResponse::Ok().finish())
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
        self, http::header::ContentType, test::{
            call_service, init_service, TestRequest
        }, web, App
    };
    use actix_http::Request;
    use actix_web::http::header;
    use dal_tx_impl::impl_transaction;
    use kernel::users::UserRole;
    use serde_json::json;
    use kernel::token::session_cache::engine_mock::PassAuthSessionCheckMock;
    use utils::errors::NanoServiceError;
    use utils::config::GetConfigVariable;
    use kernel::token::{
        token::HeaderToken,
        checks::SuperAdminRoleCheck
    };


    #[tokio::test]
    async fn test_pass() {
        struct MockDbHandle;
        struct MockConfig;

        #[impl_transaction(MockDbHandle, UnblockUser, unblock_user)]
        async fn unblock_user(user_id: i32) -> Result<bool, NanoServiceError> {
            assert_eq!(user_id, 2);
            Ok(true)
        }

        async fn run_request(req: Request) -> ServiceResponse {
            let service = unblock_user::<MockDbHandle, MockConfig, PassAuthSessionCheckMock>;
            let app = init_service(App::new().route("/unblock_user", web::post().to(service))).await;
            call_service(&app, req).await
        }

        impl GetConfigVariable for MockConfig {
            fn get_config_variable(_key: String) -> Result<String, NanoServiceError> {
                Ok("secret".to_string())
            }
        }

        let agent = "some-agent".to_string();

        let jwt: HeaderToken<MockConfig, SuperAdminRoleCheck> = HeaderToken::new(
            agent.clone(), 
            1, 
            UserRole::SuperAdmin,
        );

        let body = json!({
            "user_id": 2,
        });

        let req = TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("token", jwt.encode().unwrap()))
            .insert_header((header::USER_AGENT, agent))
            .uri("/unblock_user")
            .set_json(&body)
            .to_request();

        let resp = run_request(req).await;
        assert_eq!(resp.status(), 200);


    }
}