//! Defines the RolePermission struct for managing user roles in the system.
//!
//! This file provides data structures and utility methods for managing user role assignments and interactions
//! between the kernel workspace and the data access layer.
//!
//! ## Purpose
//! - To associate users with specific roles in the system, ensuring proper authorization and access control.

use serde::{Serialize, Deserialize};
use crate::users::UserRole;

/// Represents the schema for a new role permission entry in the system.
/// 
/// # Fields
/// * user_id - The ID of the user.
/// * role - The role assigned to the user.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewRolePermission {
    pub user_id: i32,
    pub role: UserRole,
}

impl NewRolePermission {
    /// Creates a new NewRolePermission instance.
    ///
    /// # Arguments
    /// * user_id - The ID of the user.
    /// * role - The role assigned to the user.
    ///
    /// # Returns
    /// * NewRolePermission - If valid data is provided.
    pub fn new(user_id: i32, role: UserRole) -> NewRolePermission {
        NewRolePermission { user_id, role }
    }
}


/// Represents the schema for a role permission entry in the system.
/// 
/// # Fields
/// * id - The unique identifier for the role permission entry.
/// * user_id - The ID of the user.
/// * role - The role assigned to the user.
#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow, PartialEq)]
pub struct RolePermission {
    pub id: i32,
    pub user_id: i32,
    pub role: UserRole,
}

impl RolePermission {
    /// Checks if the user has a specific role.
    ///
    /// # Arguments
    /// * required_role - The role to check against.
    ///
    /// # Returns
    /// - `true` if the user's role matches the required role.
    /// - `false` otherwise.
    pub fn has_role(&self, required_role: UserRole) -> bool {
        self.role == required_role
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_role_permission_entry() {
        let user_id = 42;
        let role = UserRole::Admin;

        let new_entry = NewRolePermission::new(user_id, role.clone());

        assert_eq!(new_entry.user_id, user_id);
        assert_eq!(new_entry.role, role);
    }

    #[test]
    fn test_role_permission_entry_has_role() {
        let entry = RolePermission {
            id: 1,
            user_id: 42,
            role: UserRole::Admin,
        };

        assert!(entry.has_role(UserRole::Admin));
        assert!(!entry.has_role(UserRole::Worker));
    }
}
