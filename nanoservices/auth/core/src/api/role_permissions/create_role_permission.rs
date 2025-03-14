//! Core logic for creating role permission entries.
//!
//! # Overview
//! This file contains the core functionality for inserting a role permission entry.
//! It defines high-level business logic that interacts with the data access layer (DAL).
//!
//! # Features
//! - Validates input data before processing.
//! - Delegates role permission creation to the database layer.
//!
//! # Notes
//! - The function is generic, allowing different database implementations to be injected for testing.

use utils::errors::NanoServiceError;
use dal::role_permissions::tx_definitions::CreateRolePermission;
use kernel::role_permissions::{NewRolePermission, RolePermission};

/// Creates a new role permission entry.
///
/// # Arguments
/// - `entry`: The `NewRolePermission` struct containing user ID and role.
///
/// # Returns
/// - `Ok(RolePermission)`: The created role permission entry.
/// - `Err(NanoServiceError)`: If the operation fails.
///
/// # Notes
/// - Uses the `CreateRolePermission` trait to perform the database transaction.
pub async fn create_role_permission<X: CreateRolePermission>(entry: NewRolePermission) -> Result<RolePermission, NanoServiceError> {
    X::create_role_permission(entry).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use dal_tx_impl::impl_transaction;
    use kernel::users::UserRole;

    struct MockDbHandle;

    #[impl_transaction(MockDbHandle, CreateRolePermission, create_role_permission)]
    async fn create_role_permission(entry: NewRolePermission) -> Result<RolePermission, NanoServiceError> {
        Ok(RolePermission {
            id: 1,
            user_id: entry.user_id,
            role: entry.role,
        })
    }

    #[tokio::test]
    async fn test_create_role_permission_ok() {
        let entry = NewRolePermission {
            user_id: 10,
            role: UserRole::Admin,
        };

        let result = create_role_permission::<MockDbHandle>(entry).await.unwrap();
        assert_eq!(result.user_id, 10);
        assert_eq!(result.role, UserRole::Admin);
    }
}
