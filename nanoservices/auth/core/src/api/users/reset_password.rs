//! Core logic for resetting a users password
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use dal::users::tx_definitions::ResetPassword;
use kernel::users::hash_password;


/// Resets a users password.
/// 
/// # Arguments
/// * `uuid` - The uuid of the user.
/// * 'new_password' - The new password for the user.
pub async fn reset_password<X>(uuid: &str, new_password: &str) -> Result<(), NanoServiceError> 
where
    X: ResetPassword
{
    let hashed_password = hash_password(new_password.to_string())?;
    match X::reset_password(uuid.to_string(), hashed_password).await {
        Ok(outcome) => {
            if outcome == false {
                return Err(NanoServiceError::new("Failed to reset password".to_string(), NanoServiceErrorStatus::Unknown));
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

    #[tokio::test]
    async fn test_pass() {
        struct MockPostgres;

        #[impl_transaction(MockPostgres, ResetPassword, reset_password)]
        async fn reset_password(uuid: String, _new_password: String) -> Result<bool, NanoServiceError> {
            assert_eq!(uuid, "test_uuid");
            Ok(true)
        }

        let outcome = reset_password::<MockPostgres>("test_uuid", "new_password").await.unwrap();
        assert_eq!(outcome, ());
    }
}