// External crates
use actix_web::{HttpResponse, web::Json};
use auth_core::api::role_permissions::update_roles::update_role_permissions as update_role_permissions_core;
use dal::role_permissions::tx_definitions::UpdateRolePermissions;
use kernel::users::UserRole;
use serde::Deserialize;
use utils::api_endpoint;


/// Incoming body for updating the roles of a user.
/// 
/// # Fields
/// - `user_id`: The ID of the user to update.
/// - `roles`: The new roles to assign to the user.
#[derive(Deserialize)]
pub struct UpdateBody {
    pub user_id: i32,
    pub roles: Vec<UserRole>
}


#[api_endpoint(token=SuperAdminRoleCheck, db_traits=[UpdateRolePermissions])]
pub async fn update_roles(body: Json<UpdateBody>) {
    let body = body.into_inner();
    let _ = update_role_permissions_core::<X>(body.user_id, body.roles).await?;
    Ok(HttpResponse::Ok().finish())  
}


#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::http::header;
    use actix_web::{
        dev::ServiceResponse,
        self, http::header::ContentType, test::{
            call_service, init_service, TestRequest
        }, web, App
    };
    use kernel::users::UserRole;
    use actix_http::Request;
    use dal_tx_impl::impl_transaction;
    use serde_json::json;
    use utils::errors::NanoServiceError;
    use utils::config::GetConfigVariable;
    use kernel::token::session_cache::engine_mock::PassAuthSessionCheckMock;
    use kernel::token::token::HeaderToken;
    use kernel::token::checks::SuperAdminRoleCheck;

    #[tokio::test]
    async fn test_update_roles_pass() {
        struct MockPostgres;
        struct MockConfig;

        #[impl_transaction(MockPostgres, UpdateRolePermissions, update_role_permissions)]
        async fn update_role_permissions(_user_id: i32, _roles: Vec<UserRole>) -> Result<(), NanoServiceError> {
            Ok(())
        }

        impl GetConfigVariable for MockConfig {
            fn get_config_variable(_key: String) -> Result<String, NanoServiceError> {
                Ok("secret".to_string())
            }
        }

        async fn run_request(req: Request) -> ServiceResponse {
            let service = update_roles::<MockPostgres, MockConfig, PassAuthSessionCheckMock>;
            let app = init_service(App::new().route("/update", web::post().to(service))).await;
            call_service(&app, req).await
        }
        let body = json!({
            "user_id": 2,
            "roles": ["Admin", "Worker"]
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
            .uri("/update")
            .set_json(&body)
            .to_request();

        let resp = run_request(req).await;
        assert_eq!(resp.status(), 200);
    }

}
