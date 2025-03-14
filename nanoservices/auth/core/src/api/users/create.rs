//! Core logic for user-related operations.
//!
//! # Overview
//! This file contains the core functionality for user-related operations, such as creating a user.
//! It defines high-level business logic that is consumed by other workspaces (e.g., networking)
//! to wrap the functions into server endpoints and mount them to the server.
//!
//! # Features
//! - Converts input schemas into user entities suitable for database operations.
//! - Delegates database transactions to data access layer (DAL) traits.
//!
//! # Notes
//! - The `create_user` function is generic, enabling flexibility with different database implementations.
//! - The tests include a mock database implementation for validation of core logic.
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use dal::users::tx_definitions::CreateUser;
use dal::role_permissions::tx_definitions::CreateRolePermission;
use dal::rate_limit_entries::tx_definitions::{
    CreateRateLimitEntry,
    UpdateRateLimitEntry,
    GetRateLimitEntry,
};
use utils::config::GetConfigVariable;
use email_core::api::mailchimp_emails::confirmation_email::send_confirmation_email;
use email_core::mailchimp_traits::mc_definitions::SendTemplate;
use kernel::users::{User, NewUserSchema};
use kernel::role_permissions::NewRolePermission;
use kernel::users::UserRole;


/// Creates a new user by converting the input schema into a `NewUser`
/// and delegating the creation transaction to the data access layer.
///
/// # Arguments
/// - `new_user_schema`: The input schema containing user details.
///
/// # Returns
/// - `Ok(User)`: The newly created user if the operation is successful.
/// - `Err(NanoServiceError)`: If an error occurs during the operation.
///
/// # Notes
/// - This function uses the `CreateUser` trait to perform the database operation.
/// - Errors during schema conversion or database transactions are propagated as `NanoServiceError`.
pub async fn create_user<X, Y, Z>(
    new_user_schema: NewUserSchema
) -> Result<User, NanoServiceError> 
where
    X: CreateUser + CreateRolePermission + CreateRateLimitEntry + UpdateRateLimitEntry + GetRateLimitEntry,
    Y: SendTemplate,
    Z: GetConfigVariable,
{
    if new_user_schema.user_role == UserRole::SuperAdmin {
        return Err(NanoServiceError::new(
            "Super admin creation is not allowed with this process".to_string(), 
            utils::errors::NanoServiceErrorStatus::Unauthorized
        ))
    }
    let new_user = new_user_schema.to_new_user()?;

    let user = X::create_user(new_user).await?;
    let role_permission = NewRolePermission{
        user_id: user.id,
        role: user.user_role.clone(),
    };
    X::create_role_permission(role_permission).await?;

    match send_confirmation_email::<X, Y, Z>(user.email.clone(), user.uuid.clone()).await {
        Ok(outcome) => {
            if outcome == false {
                return Err(NanoServiceError::new("Failed to send confirmation email due to a rate limit error".to_string(), NanoServiceErrorStatus::Unknown))
            }
        },
        Err(e) => return Err(NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown))
    };

    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dal_tx_impl::impl_transaction;
    use kernel::users::NewUser;
    use kernel::role_permissions::RolePermission;
    use kernel::rate_limit_entries::{RateLimitEntry, NewRateLimitEntry};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::LazyLock;
    use chrono::{Utc, Duration};
    use utils::config::GetConfigVariable;
    use email_core::mailchimp_helpers::mailchimp_template::Template;

    fn generate_user(user: NewUser) -> User {
        let now = chrono::Utc::now().naive_utc();
        User {
            id: 1,
            confirmed: false,
            username: user.username.clone(),
            email: user.email.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            user_role: user.user_role.clone(),
            password: user.password.clone(),
            uuid: user.uuid.clone(),
            date_created: now,
            last_logged_in: now,
            blocked: user.blocked,
        }
    }

    #[tokio::test]
    async fn test_pass() {
        static CREATE_USER_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
        static CREATE_ROLE_PERMISSION_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
        static SEND_TEMPLATE_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, CreateUser, create_user)]
        async fn create_user(user: NewUser) -> Result<User, NanoServiceError> {
            CREATE_USER_CALLED.store(true, Ordering::Relaxed);
            Ok(generate_user(user))
        }

        #[impl_transaction(MockDbHandle, CreateRolePermission, create_role_permission)]
        async fn create_role_permission(role_permission: NewRolePermission) -> Result<RolePermission, NanoServiceError> {
            CREATE_ROLE_PERMISSION_CALLED.store(true, Ordering::Relaxed);
            Ok(RolePermission{
                id: 1,
                user_id: role_permission.user_id,
                role: role_permission.role.clone()
            })
        }

        #[impl_transaction(MockDbHandle, CreateRateLimitEntry, create_rate_limit_entry)]
        async fn create_rate_limit_entry(
            new_entry: NewRateLimitEntry,
        ) -> Result<RateLimitEntry, NanoServiceError> {
            Ok(RateLimitEntry {
                id: 1,
                email: new_entry.email.clone(),
                rate_limit_period_start: Utc::now().naive_utc(),
                count: 1,
            })
        }
    
        #[impl_transaction(MockDbHandle, GetRateLimitEntry, get_rate_limit_entry)]
        async fn get_rate_limit_entry(email: String) -> Result<Option<RateLimitEntry>, NanoServiceError> {
            Ok(Some(RateLimitEntry {
                id: 1,
                email,
                rate_limit_period_start: Utc::now().naive_utc() - Duration::hours(2),
                count: 2,
            }))
        }
    
        #[impl_transaction(MockDbHandle, UpdateRateLimitEntry, update_rate_limit_entry)]
        async fn update_rate_limit_entry(
            _updated_entry: RateLimitEntry,
        ) -> Result<bool, NanoServiceError> {
            Ok(true)
        }

        struct MockMailchimpHandle;

        #[impl_transaction(MockMailchimpHandle, SendTemplate, send_template)]
        async fn send_template(_template: &Template) -> Result<bool, NanoServiceError> {
            SEND_TEMPLATE_CALLED.store(true, Ordering::Relaxed);
            Ok(true)
        }

        struct FakeConfig;

        impl GetConfigVariable for FakeConfig {
            fn get_config_variable(variable: String) -> Result<String, NanoServiceError> {
                match variable.as_str() {
                    "MAILCHIMP_API_KEY" => Ok("mock_mailchimp_api".to_string()),
                    "PRODUCTION" => Ok("true".to_string()),
                    _ => Ok("".to_string()),
                }
            }
        }

        let new_user_schema = NewUserSchema {
            username: "test".to_string(),
            email: "test@gmail.com".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            user_role: UserRole::Admin
        };

        let result = create_user::<MockDbHandle, MockMailchimpHandle, FakeConfig>(new_user_schema).await;
        match result {
            Ok(_) => {
            },
            _ => panic!("Expected user"),
        }
        assert!(CREATE_USER_CALLED.load(Ordering::Relaxed));
        assert!(CREATE_ROLE_PERMISSION_CALLED.load(Ordering::Relaxed));
        assert!(SEND_TEMPLATE_CALLED.load(Ordering::Relaxed));
    }


    #[tokio::test]
    async fn test_try_create_super_user() {
        static CREATE_USER_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
        static CREATE_ROLE_PERMISSION_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
        static SEND_TEMPLATE_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, CreateUser, create_user)]
        async fn create_user(user: NewUser) -> Result<User, NanoServiceError> {
            CREATE_USER_CALLED.store(true, Ordering::Relaxed);
            Ok(generate_user(user))
        }

        #[impl_transaction(MockDbHandle, CreateRolePermission, create_role_permission)]
        async fn create_role_permission(role_permission: NewRolePermission) -> Result<RolePermission, NanoServiceError> {
            CREATE_ROLE_PERMISSION_CALLED.store(true, Ordering::Relaxed);
            Ok(RolePermission{
                id: 1,
                user_id: role_permission.user_id,
                role: role_permission.role.clone()
            })
        }

        #[impl_transaction(MockDbHandle, CreateRateLimitEntry, create_rate_limit_entry)]
        async fn create_rate_limit_entry(
            new_entry: NewRateLimitEntry,
        ) -> Result<RateLimitEntry, NanoServiceError> {
            Ok(RateLimitEntry {
                id: 1,
                email: new_entry.email.clone(),
                rate_limit_period_start: Utc::now().naive_utc(),
                count: 1,
            })
        }
    
        #[impl_transaction(MockDbHandle, GetRateLimitEntry, get_rate_limit_entry)]
        async fn get_rate_limit_entry(email: String) -> Result<Option<RateLimitEntry>, NanoServiceError> {
            Ok(Some(RateLimitEntry {
                id: 1,
                email,
                rate_limit_period_start: Utc::now().naive_utc() - Duration::hours(2),
                count: 2,
            }))
        }
    
        #[impl_transaction(MockDbHandle, UpdateRateLimitEntry, update_rate_limit_entry)]
        async fn update_rate_limit_entry(
            _updated_entry: RateLimitEntry,
        ) -> Result<bool, NanoServiceError> {
            Ok(true)
        }

        struct MockMailchimpHandle;

        #[impl_transaction(MockMailchimpHandle, SendTemplate, send_template)]
        async fn send_template(_template: &Template) -> Result<bool, NanoServiceError> {
            SEND_TEMPLATE_CALLED.store(true, Ordering::Relaxed);
            Ok(true)
        }

        struct FakeConfig;

        impl GetConfigVariable for FakeConfig {
            fn get_config_variable(variable: String) -> Result<String, NanoServiceError> {
                match variable.as_str() {
                    "MAILCHIMP_API_KEY" => Ok("mock_mailchimp_api".to_string()),
                    "PRODUCTION" => Ok("true".to_string()),
                    _ => Ok("".to_string()),
                }
            }
        }

        let new_user_schema = NewUserSchema {
            username: "test".to_string(),
            email: "test@gmail.com".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            user_role: UserRole::SuperAdmin,
        };

        let result = create_user::<MockDbHandle, MockMailchimpHandle, FakeConfig>(new_user_schema).await;
        match result {
            Err(e) => {
                assert_eq!(e.status, utils::errors::NanoServiceErrorStatus::Unauthorized);
                assert_eq!(e.message, "Super admin creation is not allowed with this process");
            }
            _ => panic!("Expected error"),
        }
        assert!(!CREATE_USER_CALLED.load(Ordering::Relaxed));
        assert!(!SEND_TEMPLATE_CALLED.load(Ordering::Relaxed));
        assert!(!CREATE_ROLE_PERMISSION_CALLED.load(Ordering::Relaxed));
    }
}