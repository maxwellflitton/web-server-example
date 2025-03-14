// External crates
use actix_web::{HttpResponse, web::Json};
use auth_core::api::role_permissions::delete_role_permission::delete_role_permission as delete_role_permission_core;
use dal::role_permissions::tx_definitions::DeleteRolePermission;
use kernel::users::UserRole;
use serde::Deserialize;
use utils::compile_api;
use kernel::token::checks::SuperAdminRoleCheck;


#[derive(Deserialize)]
pub struct DeleteRoleBody {
    pub user_id: i32, 
    pub role: UserRole
}


compile_api!(
    TOKEN(SuperAdminRoleCheck),
    |body: Json<DeleteRoleBody>| {
        let DeleteRoleBody { user_id, role } = body.into_inner(); // Fully consume body
        let _ = delete_role_permission_core::<X>(user_id, role).await?;
        Ok(HttpResponse::Ok().finish())
    }, 
    remove_role, 
    DeleteRolePermission
);


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
    use utils::config::GetConfigVariable;
    use utils::errors::NanoServiceError;
    use kernel::token::token::HeaderToken;
    use kernel::token::session_cache::engine_mock::PassAuthSessionCheckMock;

    #[tokio::test]
    async fn test_assign_role_pass() {
        struct MockPostgres;
        struct MockConfig;

        #[impl_transaction(MockPostgres, DeleteRolePermission, delete_role_permission)]
        async fn delete_role_permission(_user_id: i32, _role: UserRole) -> Result<bool, NanoServiceError> {
            Ok(true)
        }

        impl GetConfigVariable for MockConfig {
            fn get_config_variable(_key: String) -> Result<String, NanoServiceError> {
                Ok("secret".to_string())
            }
        }

        async fn run_request(req: Request) -> ServiceResponse {
            let service = remove_role::<MockPostgres, MockConfig, PassAuthSessionCheckMock>;
            let app = init_service(App::new().route("/remove_role", web::post().to(service))).await;
            call_service(&app, req).await
        }
        let body = json!({
            "user_id": 2,
            "role": "Admin"
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
            .uri("/remove_role")
            .set_json(&body)
            .to_request();

        let resp = run_request(req).await;
        assert_eq!(resp.status(), 200);
    }

}
