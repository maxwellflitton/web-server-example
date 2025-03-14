//! Core functionality for the authentication nanoservice.
//!
//! This module provides core-level functions, including creating a super user, which are independent
//! of networking and handle the business logic.
use kernel::users::{NewUser, User, UserRole};
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use dal::users::tx_definitions::CreateUser;
use dal::role_permissions::tx_definitions::CreateRolePermission;
use kernel::role_permissions::NewRolePermission;
use dal::rate_limit_entries::tx_definitions::{
    CreateRateLimitEntry,
    UpdateRateLimitEntry,
    GetRateLimitEntry,
};
use utils::config::GetConfigVariable;
use email_core::api::mailchimp_emails::confirmation_email::send_confirmation_email;
use email_core::mailchimp_traits::mc_definitions::SendTemplate;


/// Creates a super user in the system.
///
/// # Arguments
/// - `email`: The email address of the super user.
/// - `password`: The plaintext password for the super user.
/// - `first_name`: The first name of the super user.
/// - `last_name`: The last name of the super user.
///
/// # Returns
/// - `Ok(User)`: The newly created super user if successful.
/// - `Err(NanoServiceError)`: If an error occurs during user creation.
pub async fn create_super_user<X, Y, Z>(
    username: String,
    email: String,
    first_name: String,
    last_name: String,
    password: String,
) -> Result<User, NanoServiceError> 
where
    X: CreateUser + CreateRolePermission + CreateRateLimitEntry + UpdateRateLimitEntry + GetRateLimitEntry,
    Y: SendTemplate,
    Z: GetConfigVariable,
{
    // Create a `NewUser` object with the SuperAdmin role
    let new_user = NewUser::new(
        username,
        email,
        first_name,
        last_name,
        UserRole::SuperAdmin,
        password,
    )?;

    let super_admins = ["maxwellflitton@gmail.com", "raf@gmail.com", "zak@gmail.com"];

    if super_admins.contains(&new_user.email.as_str()) == false {
        return Err(NanoServiceError::new(
            format!("email: {} is not allowed to be a super admin", new_user.email),
            NanoServiceErrorStatus::Unauthorized,
        ));
    }
    println!("Creating super user: {}", new_user.email);

    // Use the CreateUser trait to insert the user into the database
    let user = X::create_user(new_user).await.map_err(|e| {
        NanoServiceError::new(
            format!("Failed to create super user: {}", e),
            NanoServiceErrorStatus::Unknown,
        )
    })?;

    let new_role_permission = NewRolePermission {
        user_id: user.id,
        role: UserRole::SuperAdmin,
    };

    // Create a role permission for the super admin
    let _ = X::create_role_permission(new_role_permission).await.map_err(|e| {
        NanoServiceError::new(
            format!("Failed to create role permission for super user: {}", e),
            NanoServiceErrorStatus::Unknown,
        )
    })?;

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
    use kernel::role_permissions::RolePermission;
    use kernel::rate_limit_entries::{RateLimitEntry, NewRateLimitEntry};
    use chrono::{Utc, Duration};
    use utils::config::GetConfigVariable;
    use email_core::mailchimp_helpers::mailchimp_template::Template;

    struct MockDbHandleOK;

    #[impl_transaction(MockDbHandleOK, CreateUser, create_user)]
    async fn create_user(user: NewUser) -> Result<User, NanoServiceError> {
        Ok(User {
            id: 1, // Mock ID
            confirmed: false,
            username: user.username.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            user_role: user.user_role.clone(),
            date_created: Utc::now().naive_utc(),
            last_logged_in: Utc::now().naive_utc(),
            blocked: user.blocked,
            uuid: user.uuid.clone(),
        })
    }

    #[impl_transaction(MockDbHandleOK, CreateRolePermission, create_role_permission)]
    async fn create_role_permission(role_permission: NewRolePermission) -> Result<RolePermission, NanoServiceError> {
        Ok(RolePermission {
            id: 1, // Mock ID
            user_id: role_permission.user_id,
            role: role_permission.role.clone(),
        })
    }

    #[impl_transaction(MockDbHandleOK, CreateRateLimitEntry, create_rate_limit_entry)]
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

    #[impl_transaction(MockDbHandleOK, GetRateLimitEntry, get_rate_limit_entry)]
    async fn get_rate_limit_entry(email: String) -> Result<Option<RateLimitEntry>, NanoServiceError> {
        Ok(Some(RateLimitEntry {
            id: 1,
            email,
            rate_limit_period_start: Utc::now().naive_utc() - Duration::hours(2),
            count: 2,
        }))
    }

    #[impl_transaction(MockDbHandleOK, UpdateRateLimitEntry, update_rate_limit_entry)]
    async fn update_rate_limit_entry(
        _updated_entry: RateLimitEntry,
    ) -> Result<bool, NanoServiceError> {
        Ok(true)
    }

    struct MockMailchimpHandle;

    #[impl_transaction(MockMailchimpHandle, SendTemplate, send_template)]
    async fn send_template(_template: &Template) -> Result<bool, NanoServiceError> {
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


    #[tokio::test]
    async fn test_create_super_user_ok() {
        // Use a valid super admin email
        let username = "superadmin_user".to_string();
        let email = "zak@gmail.com".to_string();
        let first_name = "John".to_string();
        let last_name = "Doe".to_string();
        let password = "securepassword".to_string();

        // Call `create_super_user` with valid input
        let result = create_super_user::<MockDbHandleOK, MockMailchimpHandle, FakeConfig>(
            username.clone(),
            email.clone(),
            first_name.clone(),
            last_name.clone(),
            password.clone(),
        ).await;

        // Ensure the result is `Ok`
        assert!(result.is_ok());

        // Validate returned user data
        if let Ok(user) = result {
            assert_eq!(user.username, username);
            assert_eq!(user.email, email);
            assert_eq!(user.first_name, first_name);
            assert_eq!(user.last_name, last_name);
            assert_eq!(user.user_role, UserRole::SuperAdmin);
            assert_eq!(user.blocked, false);
        } else {
            panic!("Expected Ok(User), but got an error");
        }
    }

    #[tokio::test]
    async fn test_create_super_user_err() {
        // Use an email that is NOT allowed to be a super admin
        let username = "testuser".to_string();
        let email = "test@example.com".to_string();
        let first_name = "John".to_string();
        let last_name = "Doe".to_string();
        let password = "securepassword".to_string();

        // Call `create_super_user` with an unauthorized email
        let result = create_super_user::<MockDbHandleOK, MockMailchimpHandle, FakeConfig>(
            username.clone(),
            email.clone(),
            first_name.clone(),
            last_name.clone(),
            password.clone(),
        ).await;

        // Ensure the result is `Err`
        assert!(result.is_err());

        // Validate error type and message
        match result {
            Err(e) => {
                assert_eq!(e.status, NanoServiceErrorStatus::Unauthorized);
                assert_eq!(e.message, format!("email: {} is not allowed to be a super admin", email));
            }
            _ => panic!("Expected error, but got Ok"),
        }
    }

}
