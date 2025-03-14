//! Implements the `SendTemplate` trait for `MailchimpDescriptor`.
//!
//! # Overview
//! This file provides the implementation for sending templated emails via Mailchimp.
//! The `MailchimpDescriptor` handles API interactions, and errors are managed with `NanoServiceError`.

use crate::mailchimp_traits::mc_definitions::{MailchimpDescriptor, SendTemplate};
use crate::mailchimp_helpers::mailchimp_template::Template;
use dal_tx_impl::impl_transaction;
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use reqwest::Client;

/// Implements the `SendTemplate` trait for `MailchimpDescriptor`.
/// Sends an email using a Mailchimp template and returns `true` if successful.
#[impl_transaction(MailchimpDescriptor, SendTemplate, send_template)]
async fn send_template(template: &Template) -> Result<bool, NanoServiceError> {    
    let client = Client::new();
    let response = client
        .post("https://mandrillapp.com/api/1.0/messages/send-template")
        .json(template)
        .send()
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to send HTTP request: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;

    if response.status() == 200 {
        Ok(true)
    } else {
        Err(NanoServiceError::new(
            format!("Failed to send email. HTTP Status: {}", response.status()),
            NanoServiceErrorStatus::Unknown,
        ))
    }
}

