//! Core logic for managing rate limits.
//!
//! # Overview
//! This file provides a single method, `manage_rate_limit`, which encapsulates the core functionality 
//! for creating, retrieving, and updating rate limit entries. It directly interacts with the data 
//! access layer (DAL) traits (`CreateRateLimitEntry`, `UpdateRateLimitEntry`, `GetRateLimitEntry`) 
//! to perform the necessary database operations. The function handles rate-limiting logic such as 
//! checking whether an email is within the allowed rate limit period and incrementing usage counts.

use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use dal::rate_limit_entries::tx_definitions::{
    CreateRateLimitEntry, UpdateRateLimitEntry, GetRateLimitEntry,
};
use kernel::rate_limit_entries::NewRateLimitEntry;

/// Manages the rate limit for a given email.
///
/// This function retrieves the existing rate limit entry for an email, updates it if it exists, 
/// or creates a new entry if none is found. It handles the following cases:
/// - If the email is within the rate limit period:
///     - If the usage exceeds the rate limit, the function returns `false`.
///     - Otherwise, it increments the usage count and updates the database.
/// - If the email is outside the rate limit period, it resets the count and starts a new period.
/// - If no entry is found, a new entry is created for the email.
///
/// # Arguments
/// - `email`: The email address to manage the rate limit for.
///
/// # Returns
/// - `Ok(true)`: If the email is within the allowed rate limit.
/// - `Ok(false)`: If the email exceeds the rate limit.
/// - `Err(NanoServiceError)`: If an error occurs during the operation.
pub async fn manage_rate_limit<X>(
    email: &str,
) -> Result<bool, NanoServiceError>
where
    X: CreateRateLimitEntry + UpdateRateLimitEntry + GetRateLimitEntry,
{
    let current_entry = X::get_rate_limit_entry(email.to_string()).await?;

    if let Some(mut entry) = current_entry {
        if entry.within_rate_limit_period_check()? {
            if entry.rate_limited_check()? {
                return Err(NanoServiceError::new(
                    "Email rate limited".to_string(),
                    NanoServiceErrorStatus::Unauthorized,
                ));
            } else {
                entry.count += 1;
            }
            
        } else {
            entry.rate_limit_period_start = chrono::Utc::now().naive_utc();
            entry.count = 1;
        }
        X::update_rate_limit_entry(entry).await?;

    } else {
        let new_entry = NewRateLimitEntry::new(email.to_string());
        X::create_rate_limit_entry(new_entry).await?;
    }

    Ok(true)
}


#[cfg(test)]
mod tests {

    use super::*;
    use chrono::{Utc, Duration};
    use dal_tx_impl::impl_transaction;
    use kernel::rate_limit_entries::*;

    // The mock handle for the case where there are no entries found
    struct MockDbHandleNoEntry;

    #[impl_transaction(MockDbHandleNoEntry, CreateRateLimitEntry, create_rate_limit_entry)]
    async fn create_rate_limit_entry(new_entry: NewRateLimitEntry) -> Result<RateLimitEntry, NanoServiceError> {
        Ok(
            RateLimitEntry {
                id: 1,
                email: new_entry.email.clone(),
                rate_limit_period_start: Utc::now().naive_utc(),
                count: 1
            }
        )
    }

    #[impl_transaction(MockDbHandleNoEntry, GetRateLimitEntry, get_rate_limit_entry)]
    async fn get_rate_limit_entry(_email: String) -> Result<Option<RateLimitEntry>, NanoServiceError> {
        Ok(None)
    }

    #[impl_transaction(MockDbHandleNoEntry, UpdateRateLimitEntry, update_rate_limit_entry)]
    async fn update_rate_limit_entry(_updated_entry: RateLimitEntry) -> Result<bool, NanoServiceError> {
        Ok(false)
    }

    // The mock handle for the case where the entry found isn't within the rate limit period
    struct MockDbHandleOutsideRateLimit;

    #[impl_transaction(MockDbHandleOutsideRateLimit, CreateRateLimitEntry, create_rate_limit_entry)]
    async fn create_rate_limit_entry(new_entry: NewRateLimitEntry) -> Result<RateLimitEntry, NanoServiceError> {
        Ok(
            RateLimitEntry {
                id: 1,
                email: new_entry.email.clone(),
                rate_limit_period_start: Utc::now().naive_utc(),
                count: 1
            }
        )
    }

    #[impl_transaction(MockDbHandleOutsideRateLimit, GetRateLimitEntry, get_rate_limit_entry)]
    async fn get_rate_limit_entry(email: String) -> Result<Option<RateLimitEntry>, NanoServiceError> {
        Ok(
            Some(RateLimitEntry {
                id: 1,
                email: email.clone(),
                rate_limit_period_start: Utc::now().naive_utc() - Duration::hours(2),
                count: 2
            })
        )
    }

    #[impl_transaction(MockDbHandleOutsideRateLimit, UpdateRateLimitEntry, update_rate_limit_entry)]
    async fn update_rate_limit_entry(_updated_entry: RateLimitEntry) -> Result<bool, NanoServiceError> {
        Ok(true)
    }

    // The mock handle for the case where the entry found is found within the rate limit period and is 
    // rate limited
    struct MockDbHandleRateLimited;

    #[impl_transaction(MockDbHandleRateLimited, CreateRateLimitEntry, create_rate_limit_entry)]
    async fn create_rate_limit_entry(new_entry: NewRateLimitEntry) -> Result<RateLimitEntry, NanoServiceError> {
        Ok(
            RateLimitEntry {
                id: 1,
                email: new_entry.email.clone(),
                rate_limit_period_start: Utc::now().naive_utc(),
                count: 1
            }
        )
    }

    #[impl_transaction(MockDbHandleRateLimited, GetRateLimitEntry, get_rate_limit_entry)]
    async fn get_rate_limit_entry(email: String) -> Result<Option<RateLimitEntry>, NanoServiceError> {
        Ok(
            Some(RateLimitEntry {
                id: 1,
                email: email.clone(),
                rate_limit_period_start: Utc::now().naive_utc() - Duration::minutes(30),
                count: 5
            })
        )
    }

    #[impl_transaction(MockDbHandleRateLimited, UpdateRateLimitEntry, update_rate_limit_entry)]
    async fn update_rate_limit_entry(_updated_entry: RateLimitEntry) -> Result<bool, NanoServiceError> {
        Ok(false)
    }

    // The mock handle for the case where the entry found is found within the rate limit period and isn't 
    // rate limited
    struct MockDbHandleNotRateLimited;

    #[impl_transaction(MockDbHandleNotRateLimited, CreateRateLimitEntry, create_rate_limit_entry)]
    async fn create_rate_limit_entry(new_entry: NewRateLimitEntry) -> Result<RateLimitEntry, NanoServiceError> {
        Ok(
            RateLimitEntry {
                id: 1,
                email: new_entry.email.clone(),
                rate_limit_period_start: Utc::now().naive_utc(),
                count: 1
            }
        )
    }

    #[impl_transaction(MockDbHandleNotRateLimited, GetRateLimitEntry, get_rate_limit_entry)]
    async fn get_rate_limit_entry(email: String) -> Result<Option<RateLimitEntry>, NanoServiceError> {
        Ok(
            Some(RateLimitEntry {
                id: 1,
                email: email.clone(),
                rate_limit_period_start: Utc::now().naive_utc() - Duration::minutes(30),
                count: 4
            })
        )
    }

    #[impl_transaction(MockDbHandleNotRateLimited, UpdateRateLimitEntry, update_rate_limit_entry)]
    async fn update_rate_limit_entry(_updated_entry: RateLimitEntry) -> Result<bool, NanoServiceError> {
        Ok(true)
    }
    
    #[tokio::test]
    async fn test_manage_rate_limit_no_entry() {
        let result = manage_rate_limit::<MockDbHandleNoEntry>("test@example.com").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[tokio::test]
    async fn test_manage_rate_limit_outside_rate_limit() {
        let result = manage_rate_limit::<MockDbHandleOutsideRateLimit>("test@example.com").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true); // Should reset the limit and allow the request
    }

    #[tokio::test]
    async fn test_manage_rate_limit_rate_limited() {
        let result = manage_rate_limit::<MockDbHandleRateLimited>("test@example.com").await;
        assert!(result.is_err()); // Now expects an error instead of Ok(false)
        
        let error = result.unwrap_err();
        assert_eq!(error.message, "Email rate limited");
    }

    #[tokio::test]
    async fn test_manage_rate_limit_not_rate_limited() {
        let result = manage_rate_limit::<MockDbHandleNotRateLimited>("test@example.com").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true); // Should allow the request since under the limit
    }
}
