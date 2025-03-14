//! Core logic for retrieving to-do items assigned to a user.
//!
//! # Overview
//! This file contains the core functionality for retrieving all to-do items assigned to a specific user.
//! It delegates the retrieval transaction to the data access layer (DAL).
//!
//! # Features
//! - Delegates the retrieval operation to the data access layer (DAL) using `GetToDoItemsForUser`.
use utils::errors::NanoServiceError;
use dal::to_do_items::tx_definitions::GetToDoItemsForUser;
use kernel::to_do_items::Todo;

/// Retrieves all to-do items assigned to a specific user.
///
/// # Arguments
/// - `user_id`: The unique identifier of the user.
///
/// # Returns
/// - `Ok(Vec<Todo>)`: A list of to-do items assigned to the user if the operation is successful.
/// - `Err(NanoServiceError)`: If an error occurs during the database transaction.
///
/// # Notes
/// - This function uses the `GetToDoItemsForUser` trait to perform the database operation.
pub async fn get_to_do_items_for_user<X: GetToDoItemsForUser>(user_id: i32) -> Result<Vec<Todo>, NanoServiceError> {
    X::get_to_do_items_for_user(user_id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use dal_tx_impl::impl_transaction;
    use chrono::Utc;

    /// Tests retrieving to-do items for a user successfully using a mock database implementation.
    #[tokio::test]
    async fn test_get_to_do_items_for_user_ok() {
        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, GetToDoItemsForUser, get_to_do_items_for_user)]
        async fn get_to_do_items_for_user(user_id: i32) -> Result<Vec<Todo>, NanoServiceError> {
            assert_eq!(user_id, 1);
            let now = Utc::now().naive_utc();
            Ok(vec![
                Todo {
                    id: 1,
                    name: "Task 1".to_string(),
                    due_date: Some(now),
                    assigned_by: 2,
                    assigned_to: user_id,
                    description: Some("Description 1".to_string()),
                    date_assigned: now,
                    date_finished: None,
                    finished: false,
                },
                Todo {
                    id: 2,
                    name: "Task 2".to_string(),
                    due_date: Some(now),
                    assigned_by: 2,
                    assigned_to: user_id,
                    description: Some("Description 2".to_string()),
                    date_assigned: now,
                    date_finished: None,
                    finished: false,
                }
            ])
        }

        let result = get_to_do_items_for_user::<MockDbHandle>(1).await.unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "Task 1");
        assert_eq!(result[1].name, "Task 2");
    }

    /// Tests error handling when the DAL returns an error during retrieval.
    #[tokio::test]
    async fn test_get_to_do_items_for_user_error() {
        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, GetToDoItemsForUser, get_to_do_items_for_user)]
        async fn get_to_do_items_for_user(_user_id: i32) -> Result<Vec<Todo>, NanoServiceError> {
            Err(NanoServiceError::new(
                "Failed to get to-do items".to_string(),
                utils::errors::NanoServiceErrorStatus::Unknown,
            ))
        }

        let result = get_to_do_items_for_user::<MockDbHandle>(1).await;

        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.status, utils::errors::NanoServiceErrorStatus::Unknown);
        assert_eq!(error.message, "Failed to get to-do items");
    }
}
