//! Defines the `RateLimitEntry` struct for managing emails in the system.
//!
//! This file provides data structures and utility methods for email management and interaction
//! between the kernel workspace and the data access layer.
//!
//! ## Purpose
//! - To enable the admin to create an invite for a worker to create an account. The account it checked
//!   against the email invite list.
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Duration};
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use std::env;


/// Represents the schema for a new email rate limit entry in the system.
/// 
/// # Fields
/// * `email` - The email address of the entry.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewRateLimitEntry {
    pub email: String,
}

impl NewRateLimitEntry {
    /// Creates a new `NewRateLimitEntry` instance.
    ///
    /// # Arguments
    /// * `email` - The email address of the new rate limit entry.
    ///
    /// # Returns
    /// * `Ok(NewRateLimitEntry)` - If the email is valid.
    /// * `Err(NanoServiceError)` - If the email is empty or invalid.
    pub fn new(email: String) -> NewRateLimitEntry {
        return NewRateLimitEntry { email }
    }
}

/// Represents the schema for an email rate limit entry in the system.
/// 
/// # Fields
/// * `email` - The email address to be added.
/// * `rate_limit_period_start` - Timestamp of the start of the current or most recent email rate limit period.
/// * `count` - The number of times that email has been used within the recent period.
#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct RateLimitEntry {
    pub id: i32,
    pub email: String,
    pub rate_limit_period_start: NaiveDateTime,
    pub count: i32
}

impl RateLimitEntry {
    /// Checks if the current time is within the rate limit period for the email.
    ///
    /// # Returns
    /// - `Ok(true)` if the current time is within the rate limit period.
    /// - `Ok(false)` if the current time is outside the rate limit period.
    /// - `Err(NanoServiceError)` if the rate limit period end calculation fails.
    ///
    /// # Notes
    /// The duration of the rate limit period is determined by the `RATE_LIMIT_PERIOD_MINUTES` environment variable,
    /// defaulting to 60 minutes if not set or invalid.
    pub fn within_rate_limit_period_check(&self) -> Result<bool, NanoServiceError> {
        let rate_limit_period_minutes: i64 = env::var("RATE_LIMIT_PERIOD_MINUTES")
            .unwrap_or("60".to_string())
            .parse()
            .unwrap_or(60);

        let rate_limit_period_end = self
            .rate_limit_period_start
            .checked_add_signed(Duration::minutes(rate_limit_period_minutes))
            .ok_or_else(|| NanoServiceError::new(
                "Error calculating the end of the rate limit period".to_string(),
                NanoServiceErrorStatus::Unknown,
            ))?;

        let current_time = chrono::Utc::now().naive_utc();

        Ok(current_time < rate_limit_period_end)
    }

    /// Checks if the rate limit for the email has been reached or exceeded.
    ///
    /// # Returns
    /// - `Ok(true)` if the rate limit has been reached or exceeded.
    /// - `Ok(false)` if the rate limit has not been reached.
    ///
    /// # Notes
    /// The rate limit count is determined by the `RATE_LIMIT` environment variable,
    /// defaulting to 5 if not set or invalid.
    pub fn rate_limited_check(&self) -> Result<bool, NanoServiceError> {
        let rate_limit: i32 = env::var("RATE_LIMIT")
            .unwrap_or("5".to_string())
            .parse()
            .unwrap_or(5);

        Ok(self.count >= rate_limit)
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use chrono::Utc;

    #[test]
    fn test_new_rate_limit_entry() {
        let email = "test@example.com".to_string();

        let new_entry = NewRateLimitEntry::new(email.clone());

        assert_eq!(new_entry.email, email);
    }

    #[test]
    fn test_within_rate_limit_period_check_true() {
        let rate_limit_period_start = Utc::now().naive_utc();
        let entry = RateLimitEntry {
            id: 1,
            email: "test@example.com".to_string(),
            rate_limit_period_start,
            count: 3,
        };

        assert!(entry.within_rate_limit_period_check().unwrap());
    }

    #[test]
    fn test_within_rate_limit_period_check_false() {
        let rate_limit_period_start = Utc::now().naive_utc() - Duration::minutes(120);
        let entry = RateLimitEntry {
            id: 1,
            email: "test@example.com".to_string(),
            rate_limit_period_start,
            count: 3,
        };

        assert!(!entry.within_rate_limit_period_check().unwrap());
    }
}


