//! Core logic for unblocking a user
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use dal::users::tx_definitions::UnblockUser;


/// Unblocks a user in the database by setting the `blocked` attribute to `true`.
/// 
/// # Arguments
/// * `user_id` - The ID of the user to block.
pub async fn unblock_user<X>(user_id: i32) -> Result<(), NanoServiceError> 
where
    X: UnblockUser
{
    match X::unblock_user(user_id).await {
        Ok(outcome) => {
            if outcome == false {
                return Err(NanoServiceError::new("Failed to unblock user".to_string(), NanoServiceErrorStatus::Unknown));
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

        #[impl_transaction(MockPostgres, UnblockUser, unblock_user)]
        async fn unblock_user(user_id: i32) -> Result<bool, NanoServiceError> {
            assert_eq!(user_id, 1);
            Ok(true)
        }

        let outcome = unblock_user::<MockPostgres>(1).await.unwrap();
        assert_eq!(outcome, ());
    }
}