//! Core logic for requesting a password reset
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use dal::users::tx_definitions::UpdateUuid;
use dal::rate_limit_entries::tx_definitions::{
    CreateRateLimitEntry,
    UpdateRateLimitEntry,
    GetRateLimitEntry,
};
use email_core::mailchimp_traits::mc_definitions::SendTemplate;
use utils::config::GetConfigVariable;
use email_core::api::mailchimp_emails::password_reset_email::send_password_reset_email;


/// Resets a users password.
/// 
/// # Arguments
/// * `email` - The email of the user.
pub async fn request_password_reset<X, Y, Z>(email: String) -> Result<(), NanoServiceError> 
where
    X: CreateRateLimitEntry + UpdateRateLimitEntry + GetRateLimitEntry + UpdateUuid,
    Y: SendTemplate,
    Z: GetConfigVariable,
{
    let new_uuid = uuid::Uuid::new_v4().to_string();
    match X::update_uuid(email.clone(), new_uuid.clone()).await {
        Ok(outcome) => {
            if outcome == false {
                return Err(NanoServiceError::new("Failed to update users uuid".to_string(), NanoServiceErrorStatus::Unknown));
            }
        },
        Err(e) => {
            return Err(e);
        }
    }

    match send_password_reset_email::<X, Y, Z>(email.clone(), new_uuid.clone()).await {
        Ok(outcome) => {
            if outcome == false {
                return Err(NanoServiceError::new("Failed to send password reset email due to a rate limit error".to_string(), NanoServiceErrorStatus::Unknown))
            }
            Ok(())
        },
        Err(e) => Err(e)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use dal_tx_impl::impl_transaction;
    use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::LazyLock;
    use chrono::{Duration, Utc};
    use kernel::rate_limit_entries::{NewRateLimitEntry, RateLimitEntry};
    use email_core::mailchimp_helpers::mailchimp_template::Template;

    // -- Atomic flags to track which calls were made --
    static UPDATE_UUID_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
    static CREATE_RATE_LIMIT_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
    static GET_RATE_LIMIT_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
    static UPDATE_RATE_LIMIT_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
    static SEND_TEMPLATE_CALLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

    // Reset all flags before each test
    fn reset_flags() {
        UPDATE_UUID_CALLED.store(false, Ordering::Relaxed);
        CREATE_RATE_LIMIT_CALLED.store(false, Ordering::Relaxed);
        GET_RATE_LIMIT_CALLED.store(false, Ordering::Relaxed);
        UPDATE_RATE_LIMIT_CALLED.store(false, Ordering::Relaxed);
        SEND_TEMPLATE_CALLED.store(false, Ordering::Relaxed);
    }

    // GetConfigVariable Mock
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

    // Mock for Rate Limit DB (Success)
    struct MockDbHandleSuccess;

    #[impl_transaction(MockDbHandleSuccess, UpdateUuid, update_uuid)]
    async fn update_uuid(email: String, _new_uuid: String) -> Result<bool, NanoServiceError> {
        UPDATE_UUID_CALLED.store(true, Ordering::Relaxed);
        match email.as_str() {
            "example@gmail.com" => Ok(true),
            "returnfalse@gmail.com" => Ok(false),
            _ => Err(NanoServiceError::new(
                "Error updating user".to_string(),
                NanoServiceErrorStatus::NotFound,
            )),
        }
    }

    #[impl_transaction(MockDbHandleSuccess, CreateRateLimitEntry, create_rate_limit_entry)]
    async fn create_rate_limit_entry(
        new_entry: NewRateLimitEntry,
    ) -> Result<RateLimitEntry, NanoServiceError> {
        CREATE_RATE_LIMIT_CALLED.store(true, Ordering::Relaxed);
        Ok(RateLimitEntry {
            id: 1,
            email: new_entry.email.clone(),
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
            count: 2,
        }))
    }

    #[impl_transaction(MockDbHandleSuccess, UpdateRateLimitEntry, update_rate_limit_entry)]
    async fn update_rate_limit_entry(
        _updated_entry: RateLimitEntry,
    ) -> Result<bool, NanoServiceError> {
        UPDATE_RATE_LIMIT_CALLED.store(true, Ordering::Relaxed);
        Ok(true)
    }

    // Mock for Mailchimp
    struct MockMailchimpHandleOk;

    #[impl_transaction(MockMailchimpHandleOk, SendTemplate, send_template)]
    async fn send_template(_template: &Template) -> Result<bool, NanoServiceError> {
        SEND_TEMPLATE_CALLED.store(true, Ordering::Relaxed);
        Ok(true)
    }

    struct MockMailchimpHandleReturnFalse;

    #[impl_transaction(MockMailchimpHandleReturnFalse, SendTemplate, send_template)]
    async fn send_template(_template: &Template) -> Result<bool, NanoServiceError> {
        SEND_TEMPLATE_CALLED.store(true, Ordering::Relaxed);
        Ok(false)
    }

    struct MockMailchimpHandleError;

    #[impl_transaction(MockMailchimpHandleError, SendTemplate, send_template)]
    async fn send_template(_template: &Template) -> Result<bool, NanoServiceError> {
        SEND_TEMPLATE_CALLED.store(true, Ordering::Relaxed);
        Err(NanoServiceError::new(
            "Error sending email template".to_string(),
            NanoServiceErrorStatus::Unknown,
        ))
    }

    // ----------- Tests -----------

    #[tokio::test]
    async fn test_request_password_reset() {
        // Test success
        reset_flags();
        let result = request_password_reset::<MockDbHandleSuccess, MockMailchimpHandleOk, FakeConfig>(
            "example@gmail.com".to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert!(
            UPDATE_UUID_CALLED.load(Ordering::Relaxed),
            "update_uuid should be called in success flow"
        );
        assert!(
            !CREATE_RATE_LIMIT_CALLED.load(Ordering::Relaxed),
            "create_rate_limit_entry should be called"
        );
        assert!(
            GET_RATE_LIMIT_CALLED.load(Ordering::Relaxed),
            "get_rate_limit_entry should be called"
        );
        assert!(
            UPDATE_RATE_LIMIT_CALLED.load(Ordering::Relaxed),
            "update_rate_limit_entry should be called"
        );
        assert!(
            SEND_TEMPLATE_CALLED.load(Ordering::Relaxed),
            "send_template should be called"
        );

        // Test updating uuid returns false
        reset_flags();
        let result = request_password_reset::<MockDbHandleSuccess, MockMailchimpHandleOk, FakeConfig>(
            "returnfalse@gmail.com".to_string(),
        )
        .await;
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.status, NanoServiceErrorStatus::Unknown);
        assert_eq!(error.message, "Failed to update users uuid");

        assert!(UPDATE_UUID_CALLED.load(Ordering::Relaxed));
        assert!(!SEND_TEMPLATE_CALLED.load(Ordering::Relaxed));

        // Test update uuid error
        reset_flags();
        let result = request_password_reset::<MockDbHandleSuccess, MockMailchimpHandleOk, FakeConfig>(
            "wrongemail@gmail.com".to_string(),
        )
        .await;
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.status, NanoServiceErrorStatus::NotFound);
        assert_eq!(error.message, "Error updating user");

        assert!(UPDATE_UUID_CALLED.load(Ordering::Relaxed));
        assert!(!SEND_TEMPLATE_CALLED.load(Ordering::Relaxed));

        // Test send template returns false
        reset_flags();
        let result = request_password_reset::<MockDbHandleSuccess, MockMailchimpHandleReturnFalse, FakeConfig>(
            "example@gmail.com".to_string(),
        )
        .await;
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.status, NanoServiceErrorStatus::Unknown);
        assert_eq!(
            error.message,
            "Failed to send password reset email due to a rate limit error"
        );

        assert!(UPDATE_UUID_CALLED.load(Ordering::Relaxed));
        assert!(SEND_TEMPLATE_CALLED.load(Ordering::Relaxed));

        // Test send template error
        reset_flags();
        let result = request_password_reset::<MockDbHandleSuccess, MockMailchimpHandleError, FakeConfig>(
            "example@gmail.com".to_string(),
        )
        .await;
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.status, NanoServiceErrorStatus::Unknown);
        assert_eq!(error.message, "Error sending email template");

        assert!(UPDATE_UUID_CALLED.load(Ordering::Relaxed));
        assert!(SEND_TEMPLATE_CALLED.load(Ordering::Relaxed));
    }
}