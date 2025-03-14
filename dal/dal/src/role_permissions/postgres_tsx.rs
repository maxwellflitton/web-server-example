//! Implements transaction traits for PostgreSQL using the `SqlxPostGresDescriptor`.
//!
//! # Overview
//! This file implements the role permission related transaction traits (`CreateRolePermission`,
//! `GetRolePermissionEntries`, `DeleteRolePermission`) for PostgreSQL using the `SqlxPostGresDescriptor`.
//! Each implementation maps the transaction to a specific database operation.

use dal_tx_impl::impl_transaction;
use kernel::role_permissions::{RolePermission, NewRolePermission};
use kernel::users::UserRole;
use sqlx::Result;
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use crate::connections::sqlx_postgres::{SQLX_POSTGRES_POOL, SqlxPostGresDescriptor};
use crate::role_permissions::tx_definitions::{CreateRolePermission, GetRolePermissions, DeleteRolePermission, UpdateRolePermissions};

/// Implements the `CreateRolePermission` trait for the `SqlxPostGresDescriptor`.
///
/// Inserts a new role permission entry into the PostgreSQL database and returns the created entry.
#[impl_transaction(SqlxPostGresDescriptor, CreateRolePermission, create_role_permission)]
async fn create_role_permission(role_permission: NewRolePermission) -> Result<RolePermission, NanoServiceError> {
    let query = r#"
        INSERT INTO role_permissions (user_id, role)
        VALUES ($1, $2)
        RETURNING id, user_id, role
    "#;

    sqlx::query_as::<_, RolePermission>(query)
        .bind(role_permission.user_id)
        .bind(role_permission.role.to_string())
        .fetch_one(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to create role permission entry: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))
}

/// Implements the `GetRolePermissions` trait for the `SqlxPostGresDescriptor`.
///
/// Retrieves all role permission entries for a given user from the PostgreSQL database.
#[impl_transaction(SqlxPostGresDescriptor, GetRolePermissions, get_role_permissions)]
async fn get_role_permissions(user_id: i32) -> Result<Vec<RolePermission>, NanoServiceError> {
    let query = r#"
        SELECT id, user_id, role
        FROM role_permissions
        WHERE user_id = $1
    "#;

    let role_permissions = sqlx::query_as::<_, RolePermission>(query)
        .bind(user_id)
        .fetch_all(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to fetch role permission entries: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;
    
    Ok(role_permissions)
}

/// Implements the `DeleteRolePermission` trait for the `SqlxPostGresDescriptor`.
///
/// Deletes a specific role permission entry for a given user and role.
#[impl_transaction(SqlxPostGresDescriptor, DeleteRolePermission, delete_role_permission)]
async fn delete_role_permission(user_id: i32, role: UserRole) -> Result<bool, NanoServiceError> {
    let query = r#"
        DELETE FROM role_permissions
        WHERE user_id = $1 AND role = $2
    "#;

    let result = sqlx::query(query)
        .bind(user_id)
        .bind(role.to_string())
        .execute(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to delete role permission entry: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;

    Ok(result.rows_affected() > 0)
}


#[impl_transaction(SqlxPostGresDescriptor, UpdateRolePermissions, update_role_permissions)]
async fn update_role_permissions(user_id: i32, roles: Vec<UserRole>) -> Result<(), NanoServiceError> {
    // wipe all roles for user
    let query = r#"
        DELETE FROM role_permissions
        WHERE user_id = $1
    "#;
    let _ = sqlx::query(query)
        .bind(user_id)
        .execute(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to delete all role permissions for user: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;
    
    let user_ids = vec![user_id; roles.len()];
    let roles = roles.iter().map(|r| r.to_string()).collect::<Vec<String>>();

    // insert new roles
    let query = r#"
        INSERT INTO role_permissions (user_id, role)
        SELECT * FROM UNNEST(
            $1::INT4[],  -- Array of user IDs
            $2::VARCHAR(128)[]  -- Array of roles
        ) RETURNING id;
    "#;
    let _ = sqlx::query(query)
        .bind(user_ids)
        .bind(roles)
        .execute(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to update role permissions for user: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;
    Ok(())
}