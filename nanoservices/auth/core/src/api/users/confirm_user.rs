//! Core logic for confirming a user
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use dal::users::tx_definitions::ConfirmUser;


/// Blocks a user in the database by setting the `confirmed` attribute to `true`.
/// 
/// # Arguments
/// * `unique_id` - The unique ID of the user to confirm.
pub async fn confirm_user<X>(unique_id: &str) -> Result<(), NanoServiceError> 
where
    X: ConfirmUser
{
    match X::confirm_user(unique_id.to_string()).await {
        Ok(outcome) => {
            if outcome == false {
                return Err(NanoServiceError::new("Failed to confirm user".to_string(), NanoServiceErrorStatus::Unknown));
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

        #[impl_transaction(MockPostgres, ConfirmUser, confirm_user)]
        async fn confirm_user(unique_id: String) -> Result<bool, NanoServiceError> {
            assert_eq!(unique_id, "test_unique_id".to_string());
            Ok(true)
        }

        let outcome = confirm_user::<MockPostgres>("test_unique_id").await.unwrap();
        assert_eq!(outcome, ());
    }
}