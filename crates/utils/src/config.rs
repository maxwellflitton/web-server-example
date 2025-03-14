//! Defines extracting config variables.
use std::env;
use crate::errors::{NanoServiceError, NanoServiceErrorStatus};


/// Defines the trait for getting config variables
pub trait GetConfigVariable {

    /// Gets the config variable
    /// 
    /// # Arguments
    /// * `variable` - The name of the config variable to get
    /// 
    /// # Returns
    /// * `Result<String, String>` - The result of getting the config variable
    fn get_config_variable(variable: String) -> Result<String, NanoServiceError>;
}


/// Defines the struct for getting config variables from the environment
pub struct EnvConfig;


impl GetConfigVariable for EnvConfig {

    /// Gets the config variable from the environment
    /// 
    /// # Arguments
    /// * `variable` - The name of the config variable to get
    /// 
    /// # Returns
    /// * `Result<String, NanoServiceError>` - The result of getting the config variable
    fn get_config_variable(variable: String) -> Result<String, NanoServiceError> {
        match env::var(&variable) {
            Ok(val) => Ok(val),
            Err(_) => Err(
                NanoServiceError::new(
                    format!("{} not found in environment", variable),
                    NanoServiceErrorStatus::Unknown
                )
            )
        }
    }
}
