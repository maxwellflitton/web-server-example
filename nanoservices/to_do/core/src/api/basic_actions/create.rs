//! Core logic for creating a to-do item.
//!
//! # Overview
//! This file contains the core functionality for creating a to-do item in the system.
//! It converts the input schema into a `NewTodo` and delegates the creation transaction
//! to the data access layer (DAL).
//!
//! # Features
//! - Converts input schemas into `NewTodo` entities suitable for database operations.
//! - Delegates the creation operation to the data access layer (DAL) using `CreateToDoItem`.
use utils::errors::NanoServiceError;
use dal::to_do_items::tx_definitions::CreateToDoItem;
use kernel::to_do_items::{NewTodo, Todo};

/// Creates a new to-do item by converting the input schema into a `NewTodo`
/// and delegating the creation transaction to the data access layer.
///
/// # Arguments
/// - `new_todo`: The input schema containing the details of the to-do item.
///
/// # Returns
/// - `Ok(Todo)`: The newly created to-do item if the operation is successful.
/// - `Err(NanoServiceError)`: If an error occurs during validation or database transaction.
///
/// # Notes
/// - This function uses the `CreateToDoItem` trait to perform the database operation.
pub async fn create_to_do_item<X: CreateToDoItem>(new_todo: NewTodo) -> Result<Todo, NanoServiceError> {
    X::create_to_do_item(new_todo).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use dal_tx_impl::impl_transaction;
    use chrono::Utc;

    /// Tests the successful creation of a to-do item using a mock database implementation.
    #[tokio::test]
    async fn test_create_to_do_item_ok() {
        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, CreateToDoItem, create_to_do_item)]
        async fn create_to_do_item(todo: NewTodo) -> Result<Todo, NanoServiceError> {
            let now = Utc::now().naive_utc();
            Ok(Todo {
                id: 1,
                name: todo.name,
                due_date: todo.due_date,
                assigned_by: todo.assigned_by,
                assigned_to: todo.assigned_to,
                description: todo.description,
                date_assigned: todo.date_assigned.unwrap_or(now),
                date_finished: None,
                finished: false,
            })
        }

        let new_todo = NewTodo {
            name: "Test Task".to_string(),
            due_date: Some(Utc::now().naive_utc()),
            assigned_by: 1,
            assigned_to: 2,
            description: Some("Test description".to_string()),
            date_assigned: Some(Utc::now().naive_utc()),
        };

        let result = create_to_do_item::<MockDbHandle>(new_todo.clone()).await.unwrap();

        assert_eq!(result.name, new_todo.name);
        assert_eq!(result.assigned_by, new_todo.assigned_by);
        assert_eq!(result.assigned_to, new_todo.assigned_to);
        assert_eq!(result.description, new_todo.description);
        assert_eq!(result.finished, false);
    }

    /// Tests error handling when the DAL returns an error.
    #[tokio::test]
    async fn test_create_to_do_item_error() {
        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, CreateToDoItem, create_to_do_item)]
        async fn create_to_do_item(_todo: NewTodo) -> Result<Todo, NanoServiceError> {
            Err(NanoServiceError::new(
                "Failed to create to-do item".to_string(),
                utils::errors::NanoServiceErrorStatus::Unknown,
            ))
        }

        let new_todo = NewTodo {
            name: "Test Task".to_string(),
            due_date: Some(Utc::now().naive_utc()),
            assigned_by: 1,
            assigned_to: 2,
            description: Some("Test description".to_string()),
            date_assigned: Some(Utc::now().naive_utc()),
        };

        let result = create_to_do_item::<MockDbHandle>(new_todo).await;

        assert!(result.is_err());
        let error = result.err().unwrap();
        assert_eq!(error.status, utils::errors::NanoServiceErrorStatus::Unknown);
        assert_eq!(error.message, "Failed to create to-do item");
    }
}
