//! Core logic for user retrieval operations.
//!
//! # Overview
//! This file provides functions to retrieve users from the database by:
//! - `id`
//! - `email`
//! - `uuid`
//!
//! # Features
//! - Uses DAL transaction traits to retrieve user data.
//! - Converts database errors to `NanoServiceError`.
//! - Supports dependency injection for easier testing and alternative data sources.
//!
//! # Notes
//! - Returns `NanoServiceError::NotFound` if a user is not found.
//! - Each function is isolated and handles errors consistently.

use dal::users::tx_definitions::{GetUser, GetUserByEmail, GetUserByUuid};
use kernel::users::User;
use utils::errors::NanoServiceError;

/// Retrieves a user by their database ID.
///
/// # Arguments
/// - `id`: The unique identifier (primary key) of the user.
///
/// # Returns
/// - `Ok(User)`: If the user is found.
/// - `Err(NanoServiceError)`: If an error occurs or the user is not found.
pub async fn get_user<X: GetUser>(id: i32) -> Result<User, NanoServiceError> {
    X::get_user(id).await
}

/// Retrieves a user by their email address.
///
/// # Arguments
/// - `email`: The email address of the user.
///
/// # Returns
/// - `Ok(User)`: If the user is found.
/// - `Err(NanoServiceError)`: If an error occurs or the user is not found.
pub async fn get_user_by_email<X: GetUserByEmail>(email: String) -> Result<User, NanoServiceError> {
    X::get_user_by_email(email).await
}

/// Retrieves a user by their UUID.
///
/// # Arguments
/// - `uuid`: The unique identifier (UUID) of the user.
///
/// # Returns
/// - `Ok(User)`: If the user is found.
/// - `Err(NanoServiceError)`: If an error occurs or the user is not found.
pub async fn get_user_by_uuid<X: GetUserByUuid>(uuid: String) -> Result<User, NanoServiceError> {
    X::get_user_by_uuid(uuid).await
}


#[cfg(test)]
mod tests {
    use super::*;
    use dal_tx_impl::impl_transaction;
    use kernel::users::{User, UserRole};
    use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::LazyLock;

    // Flags to track which transactions were called
    static GET_USER_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
    static GET_USER_BY_EMAIL_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
    static GET_USER_BY_UUID_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

    struct MockDbHandle;

    fn mock_user() -> User {
        let now = chrono::Utc::now().naive_utc();
        User {
            id: 1,
            confirmed: true,
            username: "mockuser".to_string(),
            email: "mock@example.com".to_string(),
            password: "hashedpassword".to_string(),
            first_name: "Mock".to_string(),
            last_name: "User".to_string(),
            user_role: UserRole::Admin,
            date_created: now,
            last_logged_in: now,
            blocked: false,
            uuid: "mock-uuid".to_string(),
        }
    }

    #[impl_transaction(MockDbHandle, GetUser, get_user)]
    async fn get_user(id: i32) -> Result<User, NanoServiceError> {
        GET_USER_CALLED.store(true, Ordering::Relaxed);
        match id {
            1 => Ok(mock_user()),
            _ => Err(NanoServiceError::new(
                "User not found".to_string(),
                NanoServiceErrorStatus::NotFound,
            )),
        }
    }

    #[impl_transaction(MockDbHandle, GetUserByEmail, get_user_by_email)]
    async fn get_user_by_email(email: String) -> Result<User, NanoServiceError> {
        GET_USER_BY_EMAIL_CALLED.store(true, Ordering::Relaxed);
        match email.as_str() {
            "mock@example.com" => Ok(mock_user()),
            _ => Err(NanoServiceError::new(
                "User not found".to_string(),
                NanoServiceErrorStatus::NotFound,
            )),
        }
    }

    #[impl_transaction(MockDbHandle, GetUserByUuid, get_user_by_uuid)]
    async fn get_user_by_uuid(uuid: String) -> Result<User, NanoServiceError> {
        GET_USER_BY_UUID_CALLED.store(true, Ordering::Relaxed);
        match uuid.as_str() {
            "mock-uuid" => Ok(mock_user()),
            _ => Err(NanoServiceError::new(
                "User not found".to_string(),
                NanoServiceErrorStatus::NotFound,
            )),
        }
    }

    #[tokio::test]
    async fn test_get_user_by_id_success() {
        let result = get_user::<MockDbHandle>(1).await;
        assert!(result.is_ok());
        assert!(GET_USER_CALLED.load(Ordering::Relaxed));
    }

    #[tokio::test]
    async fn test_get_user_by_id_not_found() {
        let result = get_user::<MockDbHandle>(99).await;
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.status, NanoServiceErrorStatus::NotFound);
        assert_eq!(error.message, "User not found");
    }

    #[tokio::test]
    async fn test_get_user_by_email_success() {
        let result = get_user_by_email::<MockDbHandle>("mock@example.com".to_string()).await;
        assert!(result.is_ok());
        assert!(GET_USER_BY_EMAIL_CALLED.load(Ordering::Relaxed));
    }

    #[tokio::test]
    async fn test_get_user_by_email_not_found() {
        let result = get_user_by_email::<MockDbHandle>("unknown@example.com".to_string()).await;
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.status, NanoServiceErrorStatus::NotFound);
        assert_eq!(error.message, "User not found");
    }

    #[tokio::test]
    async fn test_get_user_by_uuid_success() {
        let result = get_user_by_uuid::<MockDbHandle>("mock-uuid".to_string()).await;
        assert!(result.is_ok());
        assert!(GET_USER_BY_UUID_CALLED.load(Ordering::Relaxed));
    }

    #[tokio::test]
    async fn test_get_user_by_uuid_not_found() {
        let result = get_user_by_uuid::<MockDbHandle>("unknown-uuid".to_string()).await;
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.status, NanoServiceErrorStatus::NotFound);
        assert_eq!(error.message, "User not found");
    }
}
