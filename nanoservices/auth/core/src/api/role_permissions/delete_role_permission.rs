//! Core logic for deleting role permission entries.
//!
//! # Overview
//! This file contains the core functionality for deleting a role permission entry.
//! It ensures proper validation before delegating the deletion to the database layer.
//!
//! # Features
//! - Ensures the role exists before attempting deletion.
//! - Uses dependency injection to allow different database implementations for testing.

use utils::errors::NanoServiceError;
use dal::role_permissions::tx_definitions::DeleteRolePermission;
use kernel::users::UserRole;

/// Deletes a specific role permission for a given user.
///
/// # Arguments
/// - `user_id`: The ID of the user.
/// - `role`: The role to be deleted.
///
/// # Returns
/// - `Ok(true)`: If the deletion was successful.
/// - `Ok(false)`: If no record was found to delete.
/// - `Err(NanoServiceError)`: If an error occurs during deletion.
pub async fn delete_role_permission<X: DeleteRolePermission>(user_id: i32, role: UserRole) -> Result<bool, NanoServiceError> {
    X::delete_role_permission(user_id, role).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use dal_tx_impl::impl_transaction;

    struct MockDbHandleDeleteOK;
    struct MockDbHandleDeleteUnsuccessful;

    #[impl_transaction(MockDbHandleDeleteOK, DeleteRolePermission, delete_role_permission)]
    async fn delete_role_permission(_user_id: i32, _role: UserRole) -> Result<bool, NanoServiceError> {
        Ok(true)
    }

    #[impl_transaction(MockDbHandleDeleteUnsuccessful, DeleteRolePermission, delete_role_permission)]
    async fn delete_role_permission(_user_id: i32, _role: UserRole) -> Result<bool, NanoServiceError> {
        Ok(false)
    }

    #[tokio::test]
    async fn test_delete_role_permission_success() {
        let result = delete_role_permission::<MockDbHandleDeleteOK>(10, UserRole::Admin).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_delete_role_permission_failure() {
        let result = delete_role_permission::<MockDbHandleDeleteUnsuccessful>(99, UserRole::Worker).await.unwrap();
        assert!(!result);
    }
}
