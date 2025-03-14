//! Core logic for reassigning a to-do item to a different user.
//!
//! # Overview
//! This file contains the core functionality for reassigning a to-do item to a different user in the system.
//! It delegates the reassignment transaction to the data access layer (DAL).
//!
//! # Features
//! - Delegates the reassignment operation to the data access layer (DAL) using `ReAssignToDoItem`.
use utils::errors::NanoServiceError;
use dal::to_do_items::tx_definitions::ReAssignToDoItem;
use kernel::to_do_items::Todo;

/// Reassigns a to-do item to a different user.
///
/// # Arguments
/// - `todo_id`: The unique identifier of the to-do item to be reassigned.
/// - `new_assigned_to`: The unique identifier of the new user to assign the task to.
///
/// # Returns
/// - `Ok(Todo)`: The updated to-do item after reassignment if the operation is successful.
/// - `Err(NanoServiceError)`: If an error occurs during the database transaction.
pub async fn re_assign_to_do_item<X: ReAssignToDoItem>(todo_id: i32, new_assigned_to: i32) -> Result<Todo, NanoServiceError> {
    X::re_assign_to_do_item(todo_id, new_assigned_to).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use dal_tx_impl::impl_transaction;
    use chrono::Utc;

    /// Tests successfully reassigning a to-do item using a mock database implementation.
    #[tokio::test]
    async fn test_re_assign_to_do_item_ok() {
        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, ReAssignToDoItem, re_assign_to_do_item)]
        async fn re_assign_to_do_item(todo_id: i32, new_assigned_to: i32) -> Result<Todo, NanoServiceError> {
            assert_eq!(todo_id, 1);
            assert_eq!(new_assigned_to, 3);

            let now = Utc::now().naive_utc();
            Ok(Todo {
                id: todo_id,
                name: "Reassigned Task".to_string(),
                due_date: Some(now),
                assigned_by: 2,
                assigned_to: new_assigned_to,
                description: Some("Reassigned task description".to_string()),
                date_assigned: now,
                date_finished: None,
                finished: false,
            })
        }

        let result = re_assign_to_do_item::<MockDbHandle>(1, 3).await.unwrap();

        assert_eq!(result.id, 1);
        assert_eq!(result.assigned_to, 3);
        assert_eq!(result.name, "Reassigned Task");
    }

    /// Tests error handling when the DAL returns an error during reassignment.
    #[tokio::test]
    async fn test_re_assign_to_do_item_error() {
        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, ReAssignToDoItem, re_assign_to_do_item)]
        async fn re_assign_to_do_item(_todo_id: i32, _new_assigned_to: i32) -> Result<Todo, NanoServiceError> {
            Err(NanoServiceError::new(
                "Failed to reassign to-do item".to_string(),
                utils::errors::NanoServiceErrorStatus::Unknown,
            ))
        }

        let result = re_assign_to_do_item::<MockDbHandle>(1, 3).await;

        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.status, utils::errors::NanoServiceErrorStatus::Unknown);
        assert_eq!(error.message, "Failed to reassign to-do item");
    }
}
