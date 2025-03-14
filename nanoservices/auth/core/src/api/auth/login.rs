//! User Authentication Module
//!
//! This module provides a function to authenticate users by verifying their credentials
//! and generating authentication tokens. The authentication process includes password
//! validation and role-based access control to ensure that users have the appropriate
//! permissions before accessing the system.
//!
//! # Features
//! * Retrieves user details from the database.
//! * Verifies user passwords.
//! * Checks if the user has the required role.
//! * Generates and returns an authentication token.
use kernel::users::UserRole;
use dal::users::tx_definitions::GetUserByEmail;
use dal::role_permissions::tx_definitions::GetRolePermissions;
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use utils::config::GetConfigVariable;
use kernel::token::token::HeaderToken;
use kernel::token::checks::NoRoleCheck;
use kernel::token::session_cache::traits::SetAuthCacheSession;
use serde::{Deserialize, Serialize};


/// Represents the successful outcome of a user authentication process,
/// providing both an authentication token and the user's role.
///
/// # Fields
/// * `token` - A signed authentication token representing the user's session.
/// * `role` - The role assigned to the authenticated user.
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginReturnSchema {
    pub token: String,
    pub role: UserRole,
}

/// Authenticates a user by verifying credentials and generating an authentication token.
///
/// # Arguments
/// * `email` - The email address of the user attempting to log in.
/// * `password` - The plaintext password provided by the user.
/// * `role` - The role the user is attempting to authenticate as.
/// * `user_agent` - The user agent string from the request.
///
/// # Type Parameters
/// * `X` - A type that implements `GetUserByEmail` and `GetRolePermissions` for retrieving user data.
/// * `Y` - A type that implements `GetConfigVariable` for configuration handling.
///
/// # Returns
/// * `Ok(LoginReturnSchema)` - A signed authentication token and the user's role if login is successful.
/// * `Err(NanoServiceError)` - An error if authentication fails.
///
/// # Errors
/// * Returns `NanoServiceErrorStatus::Unauthorized` if the password is invalid.
/// * Returns `NanoServiceErrorStatus::Unauthorized` if the user does not have the required role.
pub async fn login<X, Y, Z>(email: String, password: String, role: UserRole, user_agent: String) -> Result<LoginReturnSchema, NanoServiceError> 
where
    X: GetUserByEmail + GetRolePermissions,
    Y: GetConfigVariable,
    Z: SetAuthCacheSession
{
    // Retrieve user information from the database
    let user = X::get_user_by_email(email).await?;

    if user.blocked {
        return Err(NanoServiceError::new(
            "User is blocked".to_string(), 
            NanoServiceErrorStatus::Unauthorized
        ));
    }
    if user.confirmed == false {
        return Err(NanoServiceError::new(
            "User is not confirmed".to_string(), 
            NanoServiceErrorStatus::Unauthorized
        ));
    }
    
    // Verify the provided password
    if !user.verify_password(password)? {
        return Err(NanoServiceError::new(
            "Invalid password".to_string(), 
            NanoServiceErrorStatus::Unauthorized
        ));
    }
    
    // Retrieve the roles associated with the user
    let roles: Vec<UserRole> = X::get_role_permissions(user.id).await?.into_iter().map(|r| r.role).collect();
    
    // Check if the user has the required role
    if !roles.contains(&role) {
        return Err(NanoServiceError::new(
            "User does not have the required role".to_string(), 
            NanoServiceErrorStatus::Unauthorized
        ));
    }
    
    // Generate authentication token
    let token: HeaderToken<Y, NoRoleCheck> = HeaderToken::new(user_agent, user.id, role.clone());
    
    // save to the cache session
    let _ = Z::set_auth_cache_session(&token, &token).await?;
    Ok(LoginReturnSchema { 
        token: token.encode()?,
        role: role
    })
}


#[cfg(test)]
mod tests {

    use super::*;
    use kernel::users::{User, NewUser};
    use kernel::role_permissions::RolePermission;
    use dal_tx_impl::impl_transaction;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::LazyLock;
    use kernel::token::session_cache::engine_mock::PassAuthSessionCheckMock;

    fn generate_user(password: String, user_role: UserRole) -> User {
        let new_user = NewUser::new(
            "test_username".to_string(),
            "test@gmail.com".to_string(),
            "first_name".to_string(),
            "last_name".to_string(),
            user_role,
            password
        ).unwrap();
        User {
            id: 1,
            confirmed: true,
            username: new_user.username,
            email: new_user.email,
            password: new_user.password,
            first_name: new_user.first_name,
            last_name: new_user.last_name,
            user_role: new_user.user_role,
            date_created: new_user.date_created,
            last_logged_in: new_user.last_logged_in,
            blocked: new_user.blocked,
            uuid: new_user.uuid,
        }
    }

    #[tokio::test]
    async fn test_pass() {
        struct MockPostgres;
        struct MockConfig;

        #[impl_transaction(MockPostgres, GetUserByEmail, get_user_by_email)]
        async fn get_user_by_email(email: String) -> Result<User, NanoServiceError> {
            assert_eq!(email, "test@gmail.com".to_string());
            Ok(generate_user("password".to_string(), UserRole::Admin))
        }

        #[impl_transaction(MockPostgres, GetRolePermissions, get_role_permissions)]
        async fn get_role_permissions(user_id: i32) -> Result<Vec<RolePermission>, NanoServiceError> {
            assert_eq!(user_id, 1);
            Ok(vec![RolePermission {
                id: 1,
                user_id: 1,
                role: UserRole::Admin,
            }])
        }
        impl GetConfigVariable for MockConfig {
            fn get_config_variable(_key: String) -> Result<String, NanoServiceError> {
                Ok("secret".to_string())
            }
        }

        let _ = login::<MockPostgres, MockConfig, PassAuthSessionCheckMock>(
            "test@gmail.com".to_string(),
            "password".to_string(),
            UserRole::Admin,
            "some-agent".to_string()
        ).await.unwrap();
    }

    #[tokio::test]
    async fn test_user_not_found() {

        static GET_USER_BY_EMAIL: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
        static GET_ROLE_PERMISSIONS: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

        struct MockPostgres;
        struct MockConfig;

        #[impl_transaction(MockPostgres, GetUserByEmail, get_user_by_email)]
        async fn get_user_by_email(email: String) -> Result<User, NanoServiceError> {
            GET_USER_BY_EMAIL.store(true, Ordering::Relaxed);
            assert_eq!(email, "test@gmail.com".to_string());
            Err(NanoServiceError::new("User not found".to_string(), NanoServiceErrorStatus::NotFound))
        }

        #[impl_transaction(MockPostgres, GetRolePermissions, get_role_permissions)]
        async fn get_role_permissions(user_id: i32) -> Result<Vec<RolePermission>, NanoServiceError> {
            GET_ROLE_PERMISSIONS.store(true, Ordering::Relaxed);
            assert_eq!(user_id, 1);
            Ok(vec![RolePermission {
                id: 1,
                user_id: 1,
                role: UserRole::Admin,
            }])
        }
        impl GetConfigVariable for MockConfig {
            fn get_config_variable(_key: String) -> Result<String, NanoServiceError> {
                Ok("secret".to_string())
            }
        }

        let result = login::<MockPostgres, MockConfig, PassAuthSessionCheckMock>(
            "test@gmail.com".to_string(),
            "password".to_string(),
            UserRole::Admin,
            "some-agent".to_string()
        ).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.status, NanoServiceErrorStatus::NotFound);
        assert_eq!(error.message, "User not found".to_string());
        assert!(GET_USER_BY_EMAIL.load(Ordering::Relaxed));
        assert!(!GET_ROLE_PERMISSIONS.load(Ordering::Relaxed));
    }

    #[tokio::test]
    async fn test_wrong_role() {

        static GET_USER_BY_EMAIL: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
        static GET_ROLE_PERMISSIONS: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

        struct MockPostgres;
        struct MockConfig;

        #[impl_transaction(MockPostgres, GetUserByEmail, get_user_by_email)]
        async fn get_user_by_email(email: String) -> Result<User, NanoServiceError> {
            GET_USER_BY_EMAIL.store(true, Ordering::Relaxed);
            assert_eq!(email, "test@gmail.com".to_string());
            Ok(generate_user("password".to_string(), UserRole::Admin))
        }

        #[impl_transaction(MockPostgres, GetRolePermissions, get_role_permissions)]
        async fn get_role_permissions(user_id: i32) -> Result<Vec<RolePermission>, NanoServiceError> {
            GET_ROLE_PERMISSIONS.store(true, Ordering::Relaxed);
            assert_eq!(user_id, 1);
            Ok(vec![RolePermission {
                id: 1,
                user_id: 1,
                role: UserRole::Worker,
            }])
        }
        impl GetConfigVariable for MockConfig {
            fn get_config_variable(_key: String) -> Result<String, NanoServiceError> {
                Ok("secret".to_string())
            }
        }

        let result = login::<MockPostgres, MockConfig, PassAuthSessionCheckMock>(
            "test@gmail.com".to_string(),
            "password".to_string(),
            UserRole::Admin,
            "some-agent".to_string()
        ).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.status, NanoServiceErrorStatus::Unauthorized);
        assert_eq!(error.message, "User does not have the required role".to_string());
        assert!(GET_USER_BY_EMAIL.load(Ordering::Relaxed));
        assert!(GET_ROLE_PERMISSIONS.load(Ordering::Relaxed));
    }

}
