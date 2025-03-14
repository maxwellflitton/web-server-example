//! Defines transaction traits for interacting with email-related services, including Mailchimp and email invites.
//!
//! # Overview
//! This file defines traits for sending email templates via Mailchimp and managing `EmailInvite` transactions.
//! The `define_dal_transactions` macro is used to create structured database interactions.
//!
//! ## Purpose
//! - Enable sending templated emails through Mailchimp.
//! - Provide database transaction traits for managing email invites.
//! - Support dependency injection for flexible implementations.
//!
//! ## Notes
//! - `MailchimpDescriptor` handles Mailchimp API interactions.
//! - `SendTemplate` defines the contract for sending dynamic email templates.
//! - `EmailInvite` transactions include creation, filtering, claiming, and deletion.

use crate::mailchimp_helpers::mailchimp_template::Template;
use std::future::Future;
use utils::errors::NanoServiceError;


/// Descriptor for Mailchimp API interactions.
pub struct MailchimpDescriptor;

/// Defines the contract for sending templated emails.
pub trait SendTemplate {
    fn send_template(template: &Template) -> impl Future<Output = Result<bool, NanoServiceError>> + Send;
}
