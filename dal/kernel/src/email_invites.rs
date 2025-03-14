//! Defines the `NewEmail`, `Email`, and related structs for managing emails in the system.
//!
//! This file provides data structures and utility methods for email management and interaction
//! between the kernel workspace and the data access layer.
//!
//! ## Purpose
//! - To enable the admin to create an invite for a worker to create an account. The account it checked
//!   against the email invite list.
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};

use crate::users::UserRole;

/// Represents the schema for creating a new email invite into the system.
/// 
/// # Fields
/// * `email` - The email address to be added.
/// * `role_assigned` - The role to assign to the user once the email is claimed.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewEmailInvite {
    pub email: String,
    pub role_assigned: UserRole,
}

/// Represents an email record retrieved from the database.
///
/// # Fields
/// * `id` - The unique identifier of the email in the database.
/// * `email` - The email address.
/// * `date_created` - The timestamp when the email was added.
/// * `claimed` - A boolean indicating if the email has been claimed.
/// * `date_claimed` - The timestamp when the email was claimed, if applicable.
/// * `role_assigned` - The role assigned upon claiming the email.
#[derive(Deserialize, Debug, Clone, PartialEq, sqlx::FromRow, Serialize)]
pub struct EmailInvite {
    pub id: i32,
    pub email: String,
    pub date_created: NaiveDateTime,
    pub claimed: bool,
    pub date_claimed: Option<NaiveDateTime>,
    pub role_assigned: UserRole,
}

impl EmailInvite {
    /// Marks the email as claimed and sets the `date_claimed` timestamp.
    /// 
    /// # Notes
    /// This might be removed if we can do the check and update of the claim in one query.
    ///
    /// # Returns
    /// * `Result<(), NanoServiceError>` - Indicates success or failure.
    pub fn claim(&mut self) -> Result<(), NanoServiceError> {
        if self.claimed {
            return Err(NanoServiceError::new(
                "Email is already claimed.".to_string(),
                NanoServiceErrorStatus::Unauthorized,
            ));
        }
        self.claimed = true;
        self.date_claimed = Some(chrono::Utc::now().naive_utc());
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_claim_email() {
        let mut email = EmailInvite {
            id: 1,
            email: "test@example.com".to_string(),
            date_created: chrono::Utc::now().naive_utc(),
            claimed: false,
            date_claimed: None,
            role_assigned: UserRole::Admin,
        };

        assert!(!email.claimed);

        email.claim().expect("Failed to claim email");

        assert!(email.claimed);
        assert!(email.date_claimed.is_some());
    }

    #[test]
    fn test_claim_already_claimed_email() {
        let mut email = EmailInvite {
            id: 1,
            email: "test@example.com".to_string(),
            date_created: chrono::Utc::now().naive_utc(),
            claimed: true,
            date_claimed: Some(chrono::Utc::now().naive_utc()),
            role_assigned: UserRole::Admin,
        };

        let result = email.claim();
        assert!(result.is_err());
    }
}
