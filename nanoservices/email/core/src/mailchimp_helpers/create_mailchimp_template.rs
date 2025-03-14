//! Core logic for creating and sending emails using dynamic templates.
//!
//! # Overview
//! This file contains the core functionality for dynamically generating email 
//! templates to be sent to mailchimp. 

use crate::mailchimp_helpers::mailchimp_template::{
    ToContent, 
    GlobalMergeVarsContent,
    MessageContent,
    Template,
};
use utils::{
    config::GetConfigVariable,
    errors::NanoServiceError,
};


/// Creates an email template with dynamic fields.
///
/// # Arguments
/// * `email` - The recipient's email address.
/// * `unique_id` - The unique identifier for the action (e.g., confirmation, reset password).
/// * `global_merge_var_name` - The name of the global merge variable (e.g., "CONFIRMATION_URL").
/// * `template_name` - The name of the template.
///
/// # Returns
/// * `Ok(Template)` - If the template was successfully created.
/// * `Err(NanoServiceError)` - If the Mailchimp API key is missing or invalid.
pub fn create_mailchimp_template<X: GetConfigVariable>(
    email: String, 
    unique_id: String, 
    global_merge_var_name: String,
    template_name: String,
) -> Result<Template, NanoServiceError> {
    let mailchimp_api_key = <X>::get_config_variable("MAILCHIMP_API_KEY".to_string())?;

    let to_content = ToContent::new(email, "to".to_string());
    let global_merge_vars_content = GlobalMergeVarsContent::new(global_merge_var_name, unique_id);

    let to_vec = vec![to_content];
    let global_merge_vars_vec = vec![global_merge_vars_content];

    let message_content = MessageContent::new(to_vec, global_merge_vars_vec);
    let template = Template::new(mailchimp_api_key, template_name, message_content);

    Ok(template)
}


#[cfg(test)]
mod tests {

    use super::*;
    use utils::errors::NanoServiceErrorStatus;

    struct FakeConfigWithApiKey;

    impl GetConfigVariable for FakeConfigWithApiKey {

        fn get_config_variable(variable: String) -> Result<String, NanoServiceError> {
            match variable.as_str() {
                "MAILCHIMP_API_KEY" => Ok("mock_mailchimp_api".to_string()),
                _ => Ok("".to_string())
            }
        }

    }

    struct FakeConfigNoApiKey;

    impl GetConfigVariable for FakeConfigNoApiKey {

        fn get_config_variable(_variable: String) -> Result<String, NanoServiceError> {
            return Err(
                NanoServiceError::new(
                    "MAILCHIMP_API_KEY not found in environment".to_string(),
                    NanoServiceErrorStatus::Unknown
                ));
        }
    }

    #[test]
    fn test_create_mailchimp_template_success() {
        let email = "test@example.com".to_string();
        let unique_id = "unique-id".to_string();
        let global_merge_var_name = "RESET_PASSWORD_URL".to_string();
        let template_name = "reset-password-template".to_string();

        let result = create_mailchimp_template::<FakeConfigWithApiKey>(
            email.clone(),
            unique_id.clone(),
            global_merge_var_name.clone(),
            template_name.clone(),
        );

        assert!(result.is_ok());
        let template = result.unwrap();
        assert_eq!(template.template_name, template_name);
        assert_eq!(template.message.to[0].email, email);
        assert_eq!(template.message.global_merge_vars[0].name, global_merge_var_name);
        assert_eq!(template.message.global_merge_vars[0].content, unique_id);
    }

    #[test]
    fn test_create_mailchimp_template_missing_api_key() {
        let result = create_mailchimp_template::<FakeConfigNoApiKey>(
            "test@example.com".to_string(),
            "unique-id".to_string(),
            "CONFIRMATION_URL".to_string(),
            "confirmation-template".to_string(),
        );

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.message, "MAILCHIMP_API_KEY not found in environment".to_string());
    }
}
