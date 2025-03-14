//! Core logic for marking a to-do item as complete.
//!
//! # Overview
//! This file contains the core functionality for marking a to-do item as completed in the system.
//! It delegates the completion transaction to the data access layer (DAL).
//!
//! # Features
//! - Delegates the completion operation to the data access layer (DAL) using `CompleteToDoItem`.
//!
//! # Notes
//! - Errors during database transactions are propagated as `NanoServiceError`.
//! - Unit tests include a mock database implementation to validate the core logic.
use utils::errors::NanoServiceError;
use dal::to_do_items::tx_definitions::CompleteToDoItem;
use kernel::to_do_items::Todo;

/// Marks a to-do item as complete.
///
/// # Arguments
/// - `todo_id`: The unique identifier of the to-do item to be marked as complete.
///
/// # Returns
/// - `Ok(Todo)`: The updated to-do item after completion if the operation is successful.
/// - `Err(NanoServiceError)`: If an error occurs during the database transaction.
pub async fn complete_to_do_item<X: CompleteToDoItem>(todo_id: i32) -> Result<Todo, NanoServiceError> {
    X::complete_to_do_item(todo_id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use dal_tx_impl::impl_transaction;
    use chrono::Utc;

    /// Tests successfully completing a to-do item using a mock database implementation.
    #[tokio::test]
    async fn test_complete_to_do_item_ok() {
        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, CompleteToDoItem, complete_to_do_item)]
        async fn complete_to_do_item(todo_id: i32) -> Result<Todo, NanoServiceError> {
            assert_eq!(todo_id, 1);

            let now = Utc::now().naive_utc();
            Ok(Todo {
                id: todo_id,
                name: "Completed Task".to_string(),
                due_date: Some(now),
                assigned_by: 2,
                assigned_to: 3,
                description: Some("This task has been completed.".to_string()),
                date_assigned: now,
                date_finished: Some(now),
                finished: true,
            })
        }

        let result = complete_to_do_item::<MockDbHandle>(1).await.unwrap();

        assert_eq!(result.id, 1);
        assert_eq!(result.finished, true);
        assert!(result.date_finished.is_some());
    }

    /// Tests error handling when the DAL returns an error during completion.
    #[tokio::test]
    async fn test_complete_to_do_item_error() {
        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, CompleteToDoItem, complete_to_do_item)]
        async fn complete_to_do_item(_todo_id: i32) -> Result<Todo, NanoServiceError> {
            Err(NanoServiceError::new(
                "Failed to complete to-do item".to_string(),
                utils::errors::NanoServiceErrorStatus::Unknown,
            ))
        }

        let result = complete_to_do_item::<MockDbHandle>(1).await;

        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.status, utils::errors::NanoServiceErrorStatus::Unknown);
        assert_eq!(error.message, "Failed to complete to-do item");
    }
}
