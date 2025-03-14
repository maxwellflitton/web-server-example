//! Defines transaction traits for interacting with the `RolePermission` database table.
//!
//! # Overview
//! This file uses the `define_dal_transactions` macro to create traits for database transactions
//! specific to the `RolePermission` entities. Each trait represents a distinct database operation such as
//! creating, updating, getting and deleteing role permission entries.
//!
//! ## Purpose
//! - Provide an interface for core logic to interact with the data access layer (DAL).
//! - Support dependency injection for database transaction implementations.
//!
//! ## Notes
//! - These traits are designed to be implemented by database descriptor structs, such as `SqlxPostGresDescriptor`.
use kernel::role_permissions::{RolePermission, NewRolePermission};
use kernel::users::UserRole;
use crate::define_dal_transactions;


define_dal_transactions!(
    CreateRolePermission => create_role_permission(role_permission: NewRolePermission) -> RolePermission,
    GetRolePermissions => get_role_permissions(user_id: i32) -> Vec<RolePermission>,
    DeleteRolePermission => delete_role_permission(user_id: i32, role: UserRole) -> bool,
    UpdateRolePermissions => update_role_permissions(user_id: i32, roles: Vec<UserRole>) -> ()
);
