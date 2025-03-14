//! Core logic for retrieving role permission entries.
//!
//! # Overview
//! This file contains the core functionality for fetching role permissions associated with a user.
//! It serves as an intermediary between the application logic and data access layer.
//!
//! # Features
//! - Supports fetching multiple role permissions for a user.
//!
//! # Notes
//! - Uses dependency injection for flexibility in testing and database handling.

use utils::errors::NanoServiceError;
use dal::role_permissions::tx_definitions::GetRolePermissions;
use kernel::role_permissions::RolePermission;

/// Retrieves all role permissions assigned to a specific user.
///
/// # Arguments
/// - `user_id`: The ID of the user.
///
/// # Returns
/// - `Ok(Vec<RolePermission>)`: List of role permissions if found.
/// - `Err(NanoServiceError)`: If an error occurs during retrieval.
pub async fn get_role_permissions<X: GetRolePermissions>(user_id: i32) -> Result<Vec<RolePermission>, NanoServiceError> {
    X::get_role_permissions(user_id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use dal_tx_impl::impl_transaction;
    use kernel::users::UserRole;

    struct MockDbHandleOK;
    struct MockDbHandleNoEntries;

    #[impl_transaction(MockDbHandleOK, GetRolePermissions, get_role_permissions)]
    async fn get_role_permissions(user_id: i32) -> Result<Vec<RolePermission>, NanoServiceError> {
        Ok(vec![RolePermission { id: 1, user_id, role: UserRole::Admin }])
    }

    #[impl_transaction(MockDbHandleNoEntries, GetRolePermissions, get_role_permissions)]
    async fn get_role_permissions(_user_id: i32) -> Result<Vec<RolePermission>, NanoServiceError> {
        Ok(vec![])
    }

    #[tokio::test]
    async fn test_get_role_permissions_with_data() {
        let result = get_role_permissions::<MockDbHandleOK>(10).await.unwrap();
        assert!(!result.is_empty());
        assert_eq!(result[0].role, UserRole::Admin);
    }

    #[tokio::test]
    async fn test_get_role_permissions_no_entries() {
        let result = get_role_permissions::<MockDbHandleNoEntries>(99).await.unwrap();
        assert!(result.is_empty());
    }
}
