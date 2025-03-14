//! Core logic for deleting a to-do item.
//!
//! # Overview
//! This file contains the core functionality for deleting a to-do item in the system.
//! It delegates the deletion transaction to the data access layer (DAL).
//!
//! # Features
//! - Delegates the delete operation to the data access layer (DAL) using `DeleteToDoItem`.
use utils::errors::NanoServiceError;
use dal::to_do_items::tx_definitions::DeleteToDoItem;

/// Deletes a to-do item by its ID.
///
/// # Arguments
/// - `todo_id`: The unique identifier of the to-do item to be deleted.
///
/// # Returns
/// - `Ok(true)`: If the to-do item was successfully deleted.
/// - `Ok(false)`: If the to-do item was not found or not deleted.
/// - `Err(NanoServiceError)`: If an error occurs during the database transaction.
pub async fn delete_to_do_item<X: DeleteToDoItem>(todo_id: i32) -> Result<bool, NanoServiceError> {
    X::delete_to_do_item(todo_id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use dal_tx_impl::impl_transaction;

    /// Tests the successful deletion of a to-do item using a mock database implementation.
    #[tokio::test]
    async fn test_delete_to_do_item_ok() {
        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, DeleteToDoItem, delete_to_do_item)]
        async fn delete_to_do_item(_id: i32) -> Result<bool, NanoServiceError> {
            Ok(true)
        }

        let result = delete_to_do_item::<MockDbHandle>(1).await.unwrap();
        assert_eq!(result, true);
    }

    /// Tests the case when a to-do item could not be deleted (returns false).
    #[tokio::test]
    async fn test_delete_to_do_item_not_found() {
        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, DeleteToDoItem, delete_to_do_item)]
        async fn delete_to_do_item(_id: i32) -> Result<bool, NanoServiceError> {
            Ok(false)
        }

        let result = delete_to_do_item::<MockDbHandle>(1).await.unwrap();
        assert_eq!(result, false);
    }

    /// Tests error handling when the DAL returns an error.
    #[tokio::test]
    async fn test_delete_to_do_item_error() {
        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, DeleteToDoItem, delete_to_do_item)]
        async fn delete_to_do_item(_id: i32) -> Result<bool, NanoServiceError> {
            Err(NanoServiceError::new(
                "Failed to delete to-do item".to_string(),
                utils::errors::NanoServiceErrorStatus::Unknown,
            ))
        }

        let result = delete_to_do_item::<MockDbHandle>(1).await;

        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.status, utils::errors::NanoServiceErrorStatus::Unknown);
        assert_eq!(error.message, "Failed to delete to-do item");
    }
}
