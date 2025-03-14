//! Core logic for managing rate limits and sending confirmation emails.
//!
//! # Overview
//! This file defines the `send_confirmation_email` method, which enforces email rate limits and
//! sends confirmation emails using Mailchimp templates. It interacts with the data access layer (DAL)
//! for rate-limit tracking and delegates email sending to the `SendTemplate` trait.

use utils::{
    config::GetConfigVariable,
    errors::NanoServiceError,
};
use dal::rate_limit_entries::tx_definitions::{
    CreateRateLimitEntry, UpdateRateLimitEntry, GetRateLimitEntry,
};
use crate::api::mailchimp_emails::manage_rate_limit::manage_rate_limit;
use crate::mailchimp_helpers::create_mailchimp_template::create_mailchimp_template;
use crate::mailchimp_traits::mc_definitions::SendTemplate;


/// Sends a confirmation email if within rate limits.
///
/// # Arguments
/// - `email`: The recipient's email address.
/// - `unique_id`: A unique identifier for the confirmation process.
///
/// # Returns
/// - `Ok(true)`: If the email was sent successfully.
/// - `Ok(false)`: If the email was blocked due to rate limits **OR** if the email send operation returned false.
/// - `Err(NanoServiceError)`: If an error occurs during processing.
///
/// ## Notes
/// - Calls `manage_rate_limit` before proceeding with email sending.
/// - Uses `create_mailchimp_template` to format the email content.
/// - Checks the `PRODUCTION` environment variable to determine whether to actually send the email.
pub async fn send_confirmation_email<X, Y, Z>(
    email: String,
    unique_id: String,
) -> Result<bool, NanoServiceError>
where
    X: CreateRateLimitEntry + UpdateRateLimitEntry + GetRateLimitEntry,
    Y: SendTemplate,
    Z: GetConfigVariable,
{
    let within_limits = manage_rate_limit::<X>(&email).await?;
    if !within_limits {
        return Ok(false);
    }

    let global_merge_var_name = "CONFIRMATION_URL".to_string();
    let template_name = "confirmation-email".to_string();
    let template = create_mailchimp_template::<Z>(email, unique_id, global_merge_var_name, template_name)?;

    let production = <Z>::get_config_variable("PRODUCTION".to_string())?;
    if production.to_uppercase().trim() == "TRUE" {
        Y::send_template(&template).await
    } else {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use dal_tx_impl::impl_transaction;
    use kernel::rate_limit_entries::{NewRateLimitEntry, RateLimitEntry};
    use crate::mailchimp_helpers::mailchimp_template::Template;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::LazyLock;
    use utils::errors::{NanoServiceError, NanoServiceErrorStatus};

    // Atomic flags
    static CREATE_RATE_LIMIT_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
    static GET_RATE_LIMIT_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
    static UPDATE_RATE_LIMIT_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
    static SEND_TEMPLATE_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

    // Helper to reset flags before each test
    fn reset_flags() {
        CREATE_RATE_LIMIT_CALLED.store(false, Ordering::Relaxed);
        GET_RATE_LIMIT_CALLED.store(false, Ordering::Relaxed);
        UPDATE_RATE_LIMIT_CALLED.store(false, Ordering::Relaxed);
        SEND_TEMPLATE_CALLED.store(false, Ordering::Relaxed);
    }

    // FakeConfigs for PRODUCTION = "TRUE" or "FALSE"
    struct FakeConfigProductionTrue;

    impl GetConfigVariable for FakeConfigProductionTrue {
        fn get_config_variable(variable: String) -> Result<String, NanoServiceError> {
            match variable.as_str() {
                "MAILCHIMP_API_KEY" => Ok("mock_mailchimp_api_key".to_string()),
                "PRODUCTION" => Ok("true".to_string()),
                _ => Ok("".to_string()),
            }
        }
    }

    struct FakeConfigProductionFalse;

    impl GetConfigVariable for FakeConfigProductionFalse {
        fn get_config_variable(variable: String) -> Result<String, NanoServiceError> {
            match variable.as_str() {
                "MAILCHIMP_API_KEY" => Ok("mock_mailchimp_api_key".to_string()),
                "PRODUCTION" => Ok("false".to_string()),
                _ => Ok("".to_string()),
            }
        }
    }

    // Mock DB success
    struct MockDbHandleSuccess;

    #[impl_transaction(MockDbHandleSuccess, CreateRateLimitEntry, create_rate_limit_entry)]
    async fn create_rate_limit_entry(new_entry: NewRateLimitEntry) -> Result<RateLimitEntry, NanoServiceError> {
        CREATE_RATE_LIMIT_CALLED.store(true, Ordering::Relaxed);
        Ok(RateLimitEntry {
            id: 1,
            email: new_entry.email,
            rate_limit_period_start: Utc::now().naive_utc(),
            count: 1,
        })
    }

    #[impl_transaction(MockDbHandleSuccess, GetRateLimitEntry, get_rate_limit_entry)]
    async fn get_rate_limit_entry(email: String) -> Result<Option<RateLimitEntry>, NanoServiceError> {
        GET_RATE_LIMIT_CALLED.store(true, Ordering::Relaxed);
        Ok(Some(RateLimitEntry {
            id: 1,
            email,
            rate_limit_period_start: Utc::now().naive_utc() - Duration::hours(2),
            count: 1,
        }))
    }

    #[impl_transaction(MockDbHandleSuccess, UpdateRateLimitEntry, update_rate_limit_entry)]
    async fn update_rate_limit_entry(_updated_entry: RateLimitEntry) -> Result<bool, NanoServiceError> {
        UPDATE_RATE_LIMIT_CALLED.store(true, Ordering::Relaxed);
        Ok(true)
    }

    // Mock db rate limited
    struct MockDbHandleRateLimited;

    #[impl_transaction(MockDbHandleRateLimited, CreateRateLimitEntry, create_rate_limit_entry)]
    async fn create_rate_limit_entry(new_entry: NewRateLimitEntry) -> Result<RateLimitEntry, NanoServiceError> {
        CREATE_RATE_LIMIT_CALLED.store(true, Ordering::Relaxed);
        Ok(RateLimitEntry {
            id: 1,
            email: new_entry.email,
            rate_limit_period_start: Utc::now().naive_utc(),
            count: 1,
        })
    }

    #[impl_transaction(MockDbHandleRateLimited, GetRateLimitEntry, get_rate_limit_entry)]
    async fn get_rate_limit_entry(email: String) -> Result<Option<RateLimitEntry>, NanoServiceError> {
        GET_RATE_LIMIT_CALLED.store(true, Ordering::Relaxed);
        Ok(Some(RateLimitEntry {
            id: 1,
            email,
            rate_limit_period_start: Utc::now().naive_utc(),
            count: 5,
        }))
    }

    #[impl_transaction(MockDbHandleRateLimited, UpdateRateLimitEntry, update_rate_limit_entry)]
    async fn update_rate_limit_entry(_updated_entry: RateLimitEntry) -> Result<bool, NanoServiceError> {
        UPDATE_RATE_LIMIT_CALLED.store(true, Ordering::Relaxed);
        Ok(true)
    }

    // Mailchimp returns Ok(true)
    struct MockMailchimpHandleOk;

    #[impl_transaction(MockMailchimpHandleOk, SendTemplate, send_template)]
    async fn send_template(_template: &Template) -> Result<bool, NanoServiceError> {
        SEND_TEMPLATE_CALLED.store(true, Ordering::Relaxed);
        Ok(true)
    }

    // Mailchimp returns Ok(false)
    struct MockMailchimpHandleFalse;

    #[impl_transaction(MockMailchimpHandleFalse, SendTemplate, send_template)]
    async fn send_template(_template: &Template) -> Result<bool, NanoServiceError> {
        SEND_TEMPLATE_CALLED.store(true, Ordering::Relaxed);
        Ok(false)
    }

    // Mailchimp returns an error
    struct MockMailchimpHandleError;

    #[impl_transaction(MockMailchimpHandleError, SendTemplate, send_template)]
    async fn send_template(_template: &Template) -> Result<bool, NanoServiceError> {
        SEND_TEMPLATE_CALLED.store(true, Ordering::Relaxed);
        Err(NanoServiceError::new(
            "Error sending email template".to_string(),
            NanoServiceErrorStatus::Unknown,
        ))
    }


    #[tokio::test]
    async fn test_send_confirmation_email() {
        // Test success
        reset_flags();
        let email = "success@example.com".to_string();
        let unique_id = "abc123".to_string();

        let result = send_confirmation_email::<
            MockDbHandleSuccess,
            MockMailchimpHandleOk,
            FakeConfigProductionTrue,
        >(email, unique_id)
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);

        assert!(
            GET_RATE_LIMIT_CALLED.load(Ordering::Relaxed),
            "get_rate_limit_entry should be called"
        );
        assert!(
            !CREATE_RATE_LIMIT_CALLED.load(Ordering::Relaxed),
            "create_rate_limit_entry should not be called if an entry already exists"
        );
        assert!(
            UPDATE_RATE_LIMIT_CALLED.load(Ordering::Relaxed),
            "update_rate_limit_entry should be called to increment usage"
        );
        assert!(
            SEND_TEMPLATE_CALLED.load(Ordering::Relaxed),
            "send_template should be called in a success scenario"
        );

        // Test email rate limited
        reset_flags();
        let email = "limited@example.com".to_string();
        let unique_id = "xyz999".to_string();

        let result = send_confirmation_email::<
            MockDbHandleRateLimited,
            MockMailchimpHandleOk,
            FakeConfigProductionTrue,
        >(email, unique_id)
        .await;

        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.message, "Email rate limited");
        assert_eq!(err.status, NanoServiceErrorStatus::Unauthorized);

        assert!(GET_RATE_LIMIT_CALLED.load(Ordering::Relaxed));
        assert!(!UPDATE_RATE_LIMIT_CALLED.load(Ordering::Relaxed));
        assert!(!SEND_TEMPLATE_CALLED.load(Ordering::Relaxed));
  
        // Test send template method returns false
        reset_flags();
        let email = "falsey@example.com".to_string();
        let unique_id = "id-456".to_string();

        let result = send_confirmation_email::<
            MockDbHandleSuccess,
            MockMailchimpHandleFalse,
            FakeConfigProductionTrue,
        >(email, unique_id)
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);

        assert!(GET_RATE_LIMIT_CALLED.load(Ordering::Relaxed));
        assert!(UPDATE_RATE_LIMIT_CALLED.load(Ordering::Relaxed));
        assert!(!CREATE_RATE_LIMIT_CALLED.load(Ordering::Relaxed));
        assert!(SEND_TEMPLATE_CALLED.load(Ordering::Relaxed));

        // Test mailchimp error
        reset_flags();
        let email = "error@example.com".to_string();
        let unique_id = "789".to_string();

        let result = send_confirmation_email::<
            MockDbHandleSuccess,
            MockMailchimpHandleError,
            FakeConfigProductionTrue,
        >(email, unique_id)
        .await;

        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.message, "Error sending email template");
        assert_eq!(err.status, NanoServiceErrorStatus::Unknown);

        assert!(GET_RATE_LIMIT_CALLED.load(Ordering::Relaxed));
        assert!(UPDATE_RATE_LIMIT_CALLED.load(Ordering::Relaxed));
        assert!(!CREATE_RATE_LIMIT_CALLED.load(Ordering::Relaxed));
        assert!(SEND_TEMPLATE_CALLED.load(Ordering::Relaxed));

        // Test production env variable is false
        reset_flags();
        let email = "dev@example.com".to_string();
        let unique_id = "dev111".to_string();

        let result = send_confirmation_email::<
            MockDbHandleSuccess,
            MockMailchimpHandleOk,
            FakeConfigProductionFalse,
        >(email, unique_id)
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);

        assert!(GET_RATE_LIMIT_CALLED.load(Ordering::Relaxed));
        assert!(UPDATE_RATE_LIMIT_CALLED.load(Ordering::Relaxed));
        assert!(!CREATE_RATE_LIMIT_CALLED.load(Ordering::Relaxed));
        assert!(!SEND_TEMPLATE_CALLED.load(Ordering::Relaxed));
    }
}
