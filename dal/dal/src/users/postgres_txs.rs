//! Implements transaction traits for PostgreSQL using the `SqlxPostGresDescriptor`.
//!
//! # Overview
//! This file implements user-related transaction traits (`CreateUser`, `ConfirmUser`, `GetUser`)
//! for PostgreSQL using `SqlxPostGresDescriptor`. Each implementation maps to a specific database operation.

use dal_tx_impl::impl_transaction;
use kernel::users::{NewUser, User, UserProfile, TrimmedUser, UserRole};
use kernel::role_permissions::RolePermission;
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use crate::connections::sqlx_postgres::{SQLX_POSTGRES_POOL, SqlxPostGresDescriptor};
use crate::users::tx_definitions::{
    CreateUser, ConfirmUser, GetUser, GetUserByEmail, GetUserProfileByEmail, GetAllUserProfiles, BlockUser, 
    UnblockUser, GetUserByUuid, ResetPassword, UpdateUuid, UpdateUserUsername, 
    UpdateUserEmail, UpdateUserFirstName, UpdateUserLasttName, DeleteUser
};
use sqlx::Row;
use std::collections::HashMap;

/// Implements the `CreateUser` trait for the `SqlxPostGresDescriptor`.
///
/// Inserts a new user into the PostgreSQL database and returns the created user record.
///
/// # Arguments
/// - `user`: The new user details.
///
/// # Returns
/// - `Ok(User)`: The created user record.
/// - `Err(NanoServiceError)`: If the insert operation fails.
#[impl_transaction(SqlxPostGresDescriptor, CreateUser, create_user)]
async fn create_user(user: NewUser) -> Result<User, NanoServiceError> {
    let query = r#"
        INSERT INTO users (
            username, email, first_name, last_name, user_role, password, uuid, date_created, last_logged_in, blocked, confirmed
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, NOW(), NOW(), $8, $9
        )
        RETURNING id, username, email, first_name, last_name, user_role, password, uuid, date_created, last_logged_in, blocked, confirmed
    "#;

    sqlx::query_as::<_, User>(query)
        .bind(user.username)
        .bind(user.email)
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.user_role.to_string())
        .bind(user.password)
        .bind(user.uuid)
        .bind(user.blocked)
        .bind(user.confirmed)
        .fetch_one(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to create user: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))
}

/// Implements the `ConfirmUser` trait for the `SqlxPostGresDescriptor`.
///
/// Marks a user as confirmed based on their UUID.
///
/// # Arguments
/// - `uuid`: The unique identifier of the user.
///
/// # Returns
/// - `Ok(bool)`: `true` if the update is successful, `false` otherwise.
/// - `Err(NanoServiceError)`: If the operation fails.
#[impl_transaction(SqlxPostGresDescriptor, ConfirmUser, confirm_user)]
async fn confirm_user(uuid: String) -> Result<bool, NanoServiceError> {
    let query = r#"
        UPDATE users
        SET confirmed = true
        WHERE uuid = $1
    "#;

    let result = sqlx::query(query)
        .bind(uuid)
        .execute(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to confirm user: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;

    Ok(result.rows_affected() > 0)
}

/// Implements the `GetUser` trait for the `SqlxPostGresDescriptor`.
///
/// Retrieves a user record from the database based on their ID.
///
/// # Arguments
/// - `id`: The unique identifier of the user.
///
/// # Returns
/// - `Ok(User)`: The user record.
/// - `Err(NanoServiceError)`: If the user is not found.
#[impl_transaction(SqlxPostGresDescriptor, GetUser, get_user)]
async fn get_user(id: i32) -> Result<User, NanoServiceError> {
    let query = r#"
        SELECT id, confirmed, username, email, first_name, last_name, user_role, password, uuid, date_created, last_logged_in, blocked
        FROM users
        WHERE id = $1
    "#;

    sqlx::query_as::<_, User>(query)
        .bind(id)
        .fetch_one(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to retrieve user: {}", e),
            NanoServiceErrorStatus::NotFound,
        ))
}


/// Implements the `GetUserByEmail` trait for the `SqlxPostGresDescriptor`.
/// 
/// Retrieves a user record from the database based on their email.
/// 
/// # Arguments
/// - `email`: The email of the user.
/// 
/// # Returns
/// - `Ok(User)`: The user record.
#[impl_transaction(SqlxPostGresDescriptor, GetUserByEmail, get_user_by_email)]
async fn get_user_by_email(email: String) -> Result<User, NanoServiceError> {
    let query = r#"
        SELECT id, confirmed, username, email, first_name, last_name, user_role, password, uuid, date_created, last_logged_in, blocked
        FROM users
        WHERE email = $1
    "#;

    sqlx::query_as::<_, User>(query)
        .bind(email)
        .fetch_one(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to retrieve user: {}", e),
            NanoServiceErrorStatus::NotFound,
        ))
}



/// Implements the `GetUserProfileByEmail` trait for the `SqlxPostGresDescriptor`.
/// 
/// Retrieves a user profile from the database based on their email.
/// 
/// # Arguments
/// - `email`: The email of the user.
/// 
/// # Returns
/// - `Ok(UserProfile)`: The user profile.
#[impl_transaction(SqlxPostGresDescriptor, GetUserProfileByEmail, get_user_profile_by_email)]
pub async fn get_user_profile_by_email(
    email: String,
) -> Result<UserProfile, NanoServiceError> {
    let query = r#"
        SELECT 
            users.id, users.username, users.email, users.first_name, users.last_name, users.user_role, 
            users.date_created, users.last_logged_in, users.blocked, users.uuid,
            role_permissions.id AS role_id, role_permissions.user_id, role_permissions.role
        FROM users
        LEFT JOIN role_permissions ON users.id = role_permissions.user_id
        WHERE users.email = $1
    "#;

    let rows = sqlx::query(query)
        .bind(&email)
        .fetch_all(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to retrieve user: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;

    if rows.is_empty() {
        return Err(NanoServiceError::new(
            format!("User with email {} not found", email),
            NanoServiceErrorStatus::NotFound,
        ));
    }
    let mut user_profile: Option<UserProfile> = None;

    for row in rows {
        let user_id: i32 = row.get("id");
        let role_id: Option<i32> = row.try_get("role_id").ok();
        let role: Option<String> = row.try_get("role").ok();

        if user_profile.is_none() {
            user_profile = Some(UserProfile {
                user: TrimmedUser {
                    id: user_id,
                    username: row.get("username"),
                    email: row.get("email"),
                    first_name: row.get("first_name"),
                    last_name: row.get("last_name"),
                    user_role: row.get("user_role"),
                    date_created: row.get("date_created"),
                    last_logged_in: row.get("last_logged_in"),
                    blocked: row.get("blocked"),
                    uuid: row.get("uuid"),
                    confirmed: row.get("confirmed")
                },
                role_permissions: vec![],
            });
        }

        if let (Some(role_id), Some(role)) = (role_id, role) {
            let role: UserRole = match role.parse() {
                Ok(role) => role,
                Err(_) => return Err(NanoServiceError::new(
                    format!("Invalid role: {}", role),
                    NanoServiceErrorStatus::Unknown,
                )),
            };
            if let Some(ref mut profile) = user_profile {
                profile.role_permissions.push(RolePermission {
                    id: role_id,
                    user_id,
                    role,
                });
            }
            else {
                return Err(NanoServiceError::new(
                    "Failed to retrieve user profile in role loop".to_string(),
                    NanoServiceErrorStatus::Unknown,
                ));
            }
        }
    }

    match user_profile {
        Some(profile) => Ok(profile),
        None => Err(NanoServiceError::new(
            format!("Failed to retrieve user profile for email: {}", email),
            NanoServiceErrorStatus::Unknown,
        )),
    }
}


#[impl_transaction(SqlxPostGresDescriptor, GetAllUserProfiles, get_all_user_profiles)]
pub async fn get_all_user_profiles() -> Result<Vec<UserProfile>, NanoServiceError> {
    let query = r#"
        SELECT 
            users.id, users.username, users.email, users.first_name, users.last_name, users.user_role, 
            users.date_created, users.last_logged_in, users.blocked, users.uuid, users.confirmed,
            role_permissions.id AS role_id, role_permissions.user_id, role_permissions.role
        FROM users
        LEFT JOIN role_permissions ON users.id = role_permissions.user_id
    "#;
    
    let rows = sqlx::query(query)
        .fetch_all(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to retrieve user profiles: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;
    let mut user_profiles: Vec<UserProfile> = vec![];
    let mut user_profiles_map: HashMap<i32, UserProfile> = HashMap::new();

    for row in rows {
        let user_id: i32 = row.get("id");
        let role_id: Option<i32> = row.try_get("role_id").ok();
        let role: Option<String> = row.try_get("role").ok();

        let user_profile = user_profiles_map.get_mut(&user_id);

        if user_profile.is_none() {
            let placeholder = UserProfile {
                user: TrimmedUser {
                    id: user_id,
                    username: row.get("username"),
                    email: row.get("email"),
                    first_name: row.get("first_name"),
                    last_name: row.get("last_name"),
                    user_role: row.get("user_role"),
                    date_created: row.get("date_created"),
                    last_logged_in: row.get("last_logged_in"),
                    blocked: row.get("blocked"),
                    uuid: row.get("uuid"),
                    confirmed: row.get("confirmed")
                },
                role_permissions: vec![],
            };
            user_profiles_map.insert(user_id, placeholder);
        }

        let user_profile = match user_profiles_map.get_mut(&user_id) {
            Some(profile) => profile,
            None => return Err(NanoServiceError::new(
                "Failed to retrieve user profile after being loaded".to_string(),
                NanoServiceErrorStatus::Unknown,
            )),
        };

        if let (Some(role_id), Some(role)) = (role_id, role) {
            let role: UserRole = match role.parse() {
                Ok(role) => role,
                Err(_) => return Err(NanoServiceError::new(
                    format!("Invalid role: {}", role),
                    NanoServiceErrorStatus::Unknown,
                )),
            };
            user_profile.role_permissions.push(RolePermission {
                id: role_id,
                user_id,
                role,
            });
        }
    }

    for user_profile in user_profiles_map.into_values() {
        user_profiles.push(user_profile);
    }
    Ok(user_profiles)
}


/// Implements the `BlockUser` trait for the `SqlxPostGresDescriptor`.
/// 
/// Blocks a user based on their ID.
/// 
/// # Arguments
/// - `user_id`: The ID of the user.
/// 
/// # Returns
/// - `Ok(bool)`: `true` if the update is successful, `false` otherwise.
#[impl_transaction(SqlxPostGresDescriptor, BlockUser, block_user)]
pub async fn block_user(user_id: i32) -> Result<bool, NanoServiceError> {
    let query = r#"
        UPDATE users
        SET blocked = true
        WHERE id = $1
    "#;

    let result = sqlx::query(query)
        .bind(user_id)
        .execute(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to block user: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;

    if result.rows_affected() > 1 {
        return Err(NanoServiceError::new(
            "Multiple users blocked".to_string(),
            NanoServiceErrorStatus::Unknown,
        ));
    }

    Ok(result.rows_affected() == 1)
}


/// Implements the `UnblockUser` trait for the `SqlxPostGresDescriptor`.
/// 
/// Unblocks a user based on their ID.
/// 
/// # Arguments
/// - `user_id`: The ID of the user.
/// 
/// # Returns
/// - `Ok(bool)`: `true` if the update is successful, `false` otherwise.
#[impl_transaction(SqlxPostGresDescriptor, UnblockUser, unblock_user)]
pub async fn unblock_user(user_id: i32) -> Result<bool, NanoServiceError> {
    let query = r#"
        UPDATE users
        SET blocked = false
        WHERE id = $1
    "#;

    let result = sqlx::query(query)
        .bind(user_id)
        .execute(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to unblock user: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;

    if result.rows_affected() > 1 {
        return Err(NanoServiceError::new(
            "Multiple users unblocked".to_string(),
            NanoServiceErrorStatus::Unknown,
        ));
    }

    Ok(result.rows_affected() == 1)
}


/// Implements the `GetUserByUuid` trait for the `SqlxPostGresDescriptor`.
///
/// This function retrieves a user by their UUID from the PostgreSQL database.
///
/// # Arguments
/// - `uuid`: The unique identifier of the user.
///
/// # Returns
/// - `Ok(User)`: The user record if found.
/// - `Err(NanoServiceError)`: If the user is not found or if a database error occurs.
#[impl_transaction(SqlxPostGresDescriptor, GetUserByUuid, get_user_by_uuid)]
async fn get_user_by_uuid(uuid: String) -> Result<User, NanoServiceError> {
    let query = r#"
        SELECT id, confirmed, username, email, password, 
               first_name, last_name, user_role, 
               date_created, last_logged_in, blocked, uuid
        FROM users
        WHERE uuid = $1
    "#;

    sqlx::query_as::<_, User>(query)
        .bind(uuid)
        .fetch_one(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to retrieve user by UUID: {}", e),
            NanoServiceErrorStatus::NotFound,
        ))
}


/// Implements the `UpdateUuid` trait for the `SqlxPostGresDescriptor`.
/// 
/// Resets the password for a user by their given uuid.
/// 
/// # Arguments
/// - `email`: The email of the user.
/// - `new_uuid`: The new uuid for the user.
/// 
/// # Returns
/// - `Ok(bool)`: `true` if the update is successful, `false` otherwise.
#[impl_transaction(SqlxPostGresDescriptor, UpdateUuid, update_uuid)]
pub async fn update_uuid(email: String, new_uuid: String) -> Result<bool, NanoServiceError> {
    let query = r#"
        UPDATE users
        SET uuid = $1
        WHERE email = $2
    "#;

    let result = sqlx::query(query)
        .bind(new_uuid)
        .bind(email)
        .execute(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to update uuid: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;

    if result.rows_affected() > 1 {
        return Err(NanoServiceError::new(
            "Multiple users updated".to_string(),
            NanoServiceErrorStatus::Unknown,
        ));
    }

    Ok(result.rows_affected() == 1)
}


/// Implements the `ResetPassword` trait for the `SqlxPostGresDescriptor`.
/// 
/// Resets the password for a user by their given uuid.
/// 
/// # Arguments
/// - `uuid`: The uuid of the user.
/// - `new_password`: The new password for the user.
/// 
/// # Returns
/// - `Ok(bool)`: `true` if the update is successful, `false` otherwise.
#[impl_transaction(SqlxPostGresDescriptor, ResetPassword, reset_password)]
pub async fn reset_password(uuid: String, new_password: String) -> Result<bool, NanoServiceError> {
    let query = r#"
        UPDATE users
        SET password = $1
        WHERE uuid = $2
    "#;

    let result = sqlx::query(query)
        .bind(new_password)
        .bind(uuid)
        .execute(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to reset password: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;

    if result.rows_affected() > 1 {
        return Err(NanoServiceError::new(
            "Multiple users edited".to_string(),
            NanoServiceErrorStatus::Unknown,
        ));
    }

    Ok(result.rows_affected() == 1)
}


/// Implements `UpdateUserUsername` to update the username field by user ID.
///
/// # Arguments
/// - `id`: The unique identifier of the user.
/// - `username`: New username.
///
/// # Returns
/// - `Ok(true)`: If update affected a row.
/// - `Err(NanoServiceError)`: If the operation fails.
#[impl_transaction(SqlxPostGresDescriptor, UpdateUserUsername, update_user_username)]
async fn update_user_username(id: i32, username: String) -> Result<bool, NanoServiceError> {
    let query = r#"
        UPDATE users
        SET username = $1
        WHERE id = $2
    "#;

    let result = sqlx::query(query)
        .bind(username)
        .bind(id)
        .execute(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to update username: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;

    Ok(result.rows_affected() > 0)
}

/// Implements `UpdateUserEmail` to update the email field by user ID.
///
/// # Arguments
/// - `id`: The unique identifier of the user.
/// - `email`: New email.
///
/// # Returns
/// - `Ok(true)`: If update affected a row.
/// - `Err(NanoServiceError)`: If the operation fails.
#[impl_transaction(SqlxPostGresDescriptor, UpdateUserEmail, update_user_email)]
async fn update_user_email(id: i32, email: String) -> Result<bool, NanoServiceError> {
    let query = r#"
        UPDATE users
        SET email = $1
        WHERE id = $2
    "#;

    let result = sqlx::query(query)
        .bind(email)
        .bind(id)
        .execute(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to update email: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;

    Ok(result.rows_affected() > 0)
}

/// Implements `UpdateUserFirstName` to update the first_name field by user ID.
///
/// # Arguments
/// - `id`: The unique identifier of the user.
/// - `first_name`: New first name.
///
/// # Returns
/// - `Ok(true)`: If update affected a row.
/// - `Err(NanoServiceError)`: If the operation fails.
#[impl_transaction(SqlxPostGresDescriptor, UpdateUserFirstName, update_user_first_name)]
async fn update_user_first_name(id: i32, first_name: String) -> Result<bool, NanoServiceError> {
    let query = r#"
        UPDATE users
        SET first_name = $1
        WHERE id = $2
    "#;

    let result = sqlx::query(query)
        .bind(first_name)
        .bind(id)
        .execute(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to update first name: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;

    Ok(result.rows_affected() > 0)
}

/// Implements `UpdateUserLasttName` to update the last_name field by user ID.
///
/// # Arguments
/// - `id`: The unique identifier of the user.
/// - `last_name`: New last name.
///
/// # Returns
/// - `Ok(true)`: If update affected a row.
/// - `Err(NanoServiceError)`: If the operation fails.
#[impl_transaction(SqlxPostGresDescriptor, UpdateUserLasttName, update_user_last_name)]
async fn update_user_last_name(id: i32, last_name: String) -> Result<bool, NanoServiceError> {
    let query = r#"
        UPDATE users
        SET last_name = $1
        WHERE id = $2
    "#;

    let result = sqlx::query(query)
        .bind(last_name)
        .bind(id)
        .execute(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to update last name: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;

    Ok(result.rows_affected() > 0)
}


/// Implements the `DeleteUser` transaction to delete a user by ID.
///
/// # Arguments
/// - `id`: The unique identifier of the user to delete.
///
/// # Returns
/// - `Ok(true)`: If the deletion was successful (a row was deleted).
/// - `Ok(false)`: If no user with the given ID was found.
/// - `Err(NanoServiceError)`: If the operation fails.
///
/// # Notes
/// - The deletion is a hard delete (removes the user entirely).
#[impl_transaction(SqlxPostGresDescriptor, DeleteUser, delete_user)]
async fn delete_user(id: i32) -> Result<bool, NanoServiceError> {
    let query = r#"
        DELETE FROM users
        WHERE id = $1
    "#;

    let result = sqlx::query(query)
        .bind(id)
        .execute(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to delete user: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;

    Ok(result.rows_affected() > 0)
}
