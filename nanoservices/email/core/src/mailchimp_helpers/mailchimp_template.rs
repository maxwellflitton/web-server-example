//! Core logic for email template schema and utilities.
//!
//! # Overview
//! This file defines the schemas for email templates, including recipients (`ToContent`),
//! global merge variables (`GlobalMergeVarsContent`), message content (`MessageContent`), and
//! the overarching email template (`Template`). Each schema includes methods for creation
//! and manipulation to simplify working with email-related data structures.

use serde::{Deserialize, Serialize};


/// Represents the `ToContent` schema for defining recipient information.
///
/// # Fields
/// * `email` - The recipient's email address.
/// * `type` - The type of recipient (e.g., `to`, `cc`, or `bcc`).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ToContent {
    pub email: String,
    #[serde(rename = "type")]
    pub recipient_type: String,
}

impl ToContent {
    /// Creates a new `ToContent` instance.
    ///
    /// # Arguments
    /// * `email` - The recipient's email address.
    /// * `recipient_type` - The type of recipient (e.g., `to`, `cc`, or `bcc`).
    ///
    /// # Returns
    /// A new `ToContent` instance.
    pub fn new(email: String, recipient_type: String) -> Self {
        ToContent { email, recipient_type }
    }
}


/// Represents the `GlobalMergeVarsContent` schema for global merge variables in templated content.
///
/// # Fields
/// * `name` - The name of the merge variable.
/// * `content` - The value or content of the merge variable.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GlobalMergeVarsContent {
    pub name: String,
    pub content: String,
}

impl GlobalMergeVarsContent {
    /// Creates a new `GlobalMergeVarsContent` instance.
    ///
    /// # Arguments
    /// * `name` - The name of the merge variable.
    /// * `content` - The value or content of the merge variable.
    ///
    /// # Returns
    /// A new `GlobalMergeVarsContent` instance.
    pub fn new(name: String, content: String) -> Self {
        GlobalMergeVarsContent { name, content }
    }
}


/// Represents the `MessageContent` schema for the content of an email message.
///
/// # Fields
/// * `to` - A list of recipients.
/// * `global_merge_vars` - A list of global merge variables for templated content.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MessageContent {
    pub to: Vec<ToContent>,
    pub global_merge_vars: Vec<GlobalMergeVarsContent>,
}

impl MessageContent {
    /// Creates a new `MessageContent` instance.
    ///
    /// # Arguments
    /// * `to` - A list of recipients (`Vec<ToContent>`).
    /// * `global_merge_vars` - A list of global merge variables (`Vec<GlobalMergeVarsContent>`).
    ///
    /// # Returns
    /// A new `MessageContent` instance.
    pub fn new(to: Vec<ToContent>, global_merge_vars: Vec<GlobalMergeVarsContent>) -> Self {
        MessageContent { to, global_merge_vars }
    }
}


/// Represents the `Template` schema for an email template.
///
/// # Fields
/// * `api_key` - The mailchimp api key.
/// * `template_name` - The name of the template.
/// * `template_content` - An empty vector for content that cannot contain any values (`Vec<!>`).
/// * `message` - The content of the email message.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Template {
    pub api_key: String,
    pub template_name: String,
    pub template_content: Vec<serde_json::Value>,
    pub message: MessageContent,
}

impl Template {
    /// Creates a new `Template` instance.
    ///
    /// # Arguments
    /// * `api_key` - The mailchimp api key.
    /// * `template_name` - The name of the template.
    /// * `message` - The content of the email message.
    ///
    /// # Returns
    /// A new `Template` instance.
    pub fn new(api_key: String, template_name: String, message: MessageContent) -> Self {
        Template {
            api_key,
            template_name,
            template_content: Vec::new(),
            message,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_content_new() {
        let email = "test@example.com".to_string();
        let recipient_type = "to".to_string();

        let to_content = ToContent::new(email.clone(), recipient_type.clone());

        assert_eq!(to_content.email, email);
        assert_eq!(to_content.recipient_type, recipient_type);
    }

    #[test]
    fn test_global_merge_vars_content_new() {
        let name = "username".to_string();
        let content = "JohnDoe".to_string();

        let global_merge_var = GlobalMergeVarsContent::new(name.clone(), content.clone());

        assert_eq!(global_merge_var.name, name);
        assert_eq!(global_merge_var.content, content);
    }

    #[test]
    fn test_message_content_new() {
        let to = vec![ToContent::new("test@example.com".to_string(), "to".to_string())];
        let global_merge_vars = vec![
            GlobalMergeVarsContent::new("username".to_string(), "JohnDoe".to_string()),
        ];

        let message_content = MessageContent::new(to.clone(), global_merge_vars.clone());

        assert_eq!(message_content.to, to);
        assert_eq!(message_content.global_merge_vars, global_merge_vars);
    }

    #[test]
    fn test_template_new() {
        let message = MessageContent::new(
            vec![ToContent::new("test@example.com".to_string(), "to".to_string())],
            vec![
                GlobalMergeVarsContent::new("username".to_string(), "JohnDoe".to_string()),
            ],
        );

        let template = Template::new("api_key".to_string(), "welcome_template".to_string(), message.clone());

        assert_eq!(template.api_key, "api_key");
        assert_eq!(template.template_name, "welcome_template");
        assert!(template.template_content.is_empty());
        assert_eq!(template.message, message);
    }
}

