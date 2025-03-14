// External crates
use actix_web::{HttpResponse, web::Json};
use auth_core::api::role_permissions::create_role_permission::create_role_permission as create_role_permission_core;
use dal::role_permissions::tx_definitions::CreateRolePermission;
use kernel::role_permissions::NewRolePermission;
use utils::api_endpoint;


#[api_endpoint(token=SuperAdminRoleCheck, db_traits=[CreateRolePermission])]
pub async fn assign_role(body: Json<NewRolePermission>) {
    let _ = create_role_permission_core::<X>(body.into_inner()).await?;
    Ok(HttpResponse::Created().finish())
}


#[cfg(test)]
mod tests {

    use super::*;
    use kernel::users::UserRole;
    use dal_tx_impl::impl_transaction;
    use kernel::role_permissions::RolePermission;
    use utils::errors::NanoServiceError;
    use kernel::token::session_cache::engine_mock::PassAuthSessionCheckMock;
    use kernel::token::checks::SuperAdminRoleCheck;
    use utils::send_test_request;

    #[tokio::test]
    async fn test_assign_role_pass() {
        struct MockPostgres;

        #[impl_transaction(MockPostgres, CreateRolePermission, create_role_permission)]
        async fn create_role_permission(_role_permission: NewRolePermission) -> Result<RolePermission, NanoServiceError> {
            Ok(RolePermission {
                id: 1,
                user_id: 1,
                role: UserRole::Admin,
            })
        }

        send_test_request!(
            POST, 
            "/assign_role", 
            serde_json::json!({
                "user_id": 2,
                "role": "Admin"
            }),
            SuperAdminRoleCheck,
            UserRole::SuperAdmin,
            1,
            assign_role,
            MockPostgres, MockConfig, PassAuthSessionCheckMock
        );

        let resp = send_request().await;

        assert_eq!(resp.status(), 201);
    }

}
