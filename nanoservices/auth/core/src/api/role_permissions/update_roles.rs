//! Updates the role permissions for a user.
use utils::errors::NanoServiceError;
use dal::role_permissions::tx_definitions::UpdateRolePermissions;
use kernel::users::UserRole;


/// Updates the role permissions for a user.
/// 
/// # Arguments
/// - `user_id`: The ID of the user to update.
/// - `roles`: The new roles to assign to the user.
pub async fn update_role_permissions<X: UpdateRolePermissions>(
    user_id: i32,
    roles: Vec<UserRole>
) -> Result<(), NanoServiceError> {
    X::update_role_permissions(user_id, roles).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use dal_tx_impl::impl_transaction;
    use kernel::users::UserRole;

    struct MockDbHandle;

    #[impl_transaction(MockDbHandle, UpdateRolePermissions, update_role_permissions)]
    async fn update_role_permissions(_user_id: i32, _roles: Vec<UserRole>) -> Result<(), NanoServiceError> {
        Ok(())
    }

    #[tokio::test]
    async fn test_update_role_permissions_ok() {
        let outcome = update_role_permissions::<MockDbHandle>(10, vec![UserRole::Admin]).await.unwrap();
        assert_eq!(outcome, ());
    }
}