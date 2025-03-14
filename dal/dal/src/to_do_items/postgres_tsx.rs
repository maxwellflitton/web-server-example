//! Implements transaction traits for PostgreSQL using the `SqlxPostGresDescriptor`.
//!
//! # Overview
//! This file implements the to-do item-related transaction traits (`CreateToDoItem`, `DeleteToDoItem`,
//! `GetToDoItemsForUser`, `GetPendingToDoItemsForUser`, `ReAssignToDoItem`, `CompleteToDoItem`)
//! for PostgreSQL using the `SqlxPostGresDescriptor`. Each implementation maps the transaction
//! to a specific database operation.
//!
//! # Features
//! - Uses the `impl_transaction` macro to streamline the implementation of transaction traits.
//! - Implements the database operations asynchronously.

use dal_tx_impl::impl_transaction;
use kernel::to_do_items::{NewTodo, Todo};
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use crate::connections::sqlx_postgres::{SQLX_POSTGRES_POOL, SqlxPostGresDescriptor};
use crate::to_do_items::tx_definitions::{
    CreateToDoItem, DeleteToDoItem, GetToDoItemsForUser,
    GetPendingToDoItemsForUser, ReAssignToDoItem, CompleteToDoItem
};

/// Implements the `CreateToDoItem` trait for the `SqlxPostGresDescriptor`.
///
/// # Arguments
/// - `todo`: A `NewTodo` instance containing the details of the to-do item to be created.
///
/// # Returns
/// - `Ok(Todo)`: The newly created to-do item.
/// - `Err(NanoServiceError)`: If the operation fails.
#[impl_transaction(SqlxPostGresDescriptor, CreateToDoItem, create_to_do_item)]
async fn create_to_do_item(todo: NewTodo) -> Result<Todo, NanoServiceError> {
    let query = r#"
        INSERT INTO todos (name, due_date, assigned_by, assigned_to, description, date_assigned)
        VALUES ($1, $2, $3, $4, $5, COALESCE($6, NOW()))
        RETURNING id, name, due_date, assigned_by, assigned_to, description, date_assigned, date_finished, finished
    "#;

    sqlx::query_as::<_, Todo>(query)
        .bind(todo.name)
        .bind(todo.due_date)
        .bind(todo.assigned_by)
        .bind(todo.assigned_to)
        .bind(todo.description)
        .bind(todo.date_assigned)
        .fetch_one(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(format!("Failed to create to-do item: {}", e), NanoServiceErrorStatus::Unknown))
}

/// Implements the `DeleteToDoItem` trait for the `SqlxPostGresDescriptor`.
///
/// # Arguments
/// - `id`: The unique identifier of the to-do item to delete.
///
/// # Returns
/// - `Ok(bool)`: `true` if the deletion was successful, `false` otherwise.
/// - `Err(NanoServiceError)`: If the operation fails.
#[impl_transaction(SqlxPostGresDescriptor, DeleteToDoItem, delete_to_do_item)]
async fn delete_to_do_item(id: i32) -> Result<bool, NanoServiceError> {
    let query = r#"
        DELETE FROM todos
        WHERE id = $1
    "#;

    let result = sqlx::query(query)
        .bind(id)
        .execute(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(format!("Failed to delete to-do item: {}", e), NanoServiceErrorStatus::Unknown))?;

    Ok(result.rows_affected() > 0)
}

/// Implements the `GetToDoItemsForUser` trait for the `SqlxPostGresDescriptor`.
///
/// # Arguments
/// - `user_id`: The ID of the user to retrieve to-do items for.
///
/// # Returns
/// - `Ok(Vec<Todo>)`: A list of to-do items assigned to the user.
/// - `Err(NanoServiceError)`: If the operation fails.
#[impl_transaction(SqlxPostGresDescriptor, GetToDoItemsForUser, get_to_do_items_for_user)]
async fn get_to_do_items_for_user(user_id: i32) -> Result<Vec<Todo>, NanoServiceError> {
    let query = r#"
        SELECT id, name, due_date, assigned_by, assigned_to, description, date_assigned, date_finished, finished
        FROM todos
        WHERE assigned_to = $1
    "#;

    sqlx::query_as::<_, Todo>(query)
        .bind(user_id)
        .fetch_all(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(format!("Failed to get to-do items: {}", e), NanoServiceErrorStatus::Unknown))
}

/// Implements the `GetPendingToDoItemsForUser` trait for the `SqlxPostGresDescriptor`.
///
/// # Arguments
/// - `user_id`: The ID of the user to retrieve pending to-do items for.
///
/// # Returns
/// - `Ok(Vec<Todo>)`: A list of pending to-do items assigned to the user.
/// - `Err(NanoServiceError)`: If the operation fails.
#[impl_transaction(SqlxPostGresDescriptor, GetPendingToDoItemsForUser, get_pending_to_do_items_for_user)]
async fn get_pending_to_do_items_for_user(user_id: i32) -> Result<Vec<Todo>, NanoServiceError> {
    let query = r#"
        SELECT id, name, due_date, assigned_by, assigned_to, description, date_assigned, date_finished, finished
        FROM todos
        WHERE assigned_to = $1 AND finished = false
    "#;

    sqlx::query_as::<_, Todo>(query)
        .bind(user_id)
        .fetch_all(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(format!("Failed to get pending to-do items: {}", e), NanoServiceErrorStatus::Unknown))
}

/// Implements the `ReAssignToDoItem` trait for the `SqlxPostGresDescriptor`.
///
/// # Arguments
/// - `todo_id`: The ID of the to-do item to reassign.
/// - `new_assigned_to`: The ID of the new user to assign the to-do item to.
///
/// # Returns
/// - `Ok(Todo)`: The updated to-do item after reassignment.
/// - `Err(NanoServiceError)`: If the operation fails.
#[impl_transaction(SqlxPostGresDescriptor, ReAssignToDoItem, re_assign_to_do_item)]
async fn re_assign_to_do_item(todo_id: i32, new_assigned_to: i32) -> Result<Todo, NanoServiceError> {
    let query = r#"
        UPDATE todos
        SET assigned_to = $1
        WHERE id = $2
        RETURNING id, name, due_date, assigned_by, assigned_to, description, date_assigned, date_finished, finished
    "#;

    sqlx::query_as::<_, Todo>(query)
        .bind(new_assigned_to)
        .bind(todo_id)
        .fetch_one(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(format!("Failed to re-assign to-do item: {}", e), NanoServiceErrorStatus::Unknown))
}

/// Implements the `CompleteToDoItem` trait for the `SqlxPostGresDescriptor`.
///
/// # Arguments
/// - `todo_id`: The ID of the to-do item to mark as complete.
///
/// # Returns
/// - `Ok(Todo)`: The updated to-do item after marking it complete.
/// - `Err(NanoServiceError)`: If the operation fails.
#[impl_transaction(SqlxPostGresDescriptor, CompleteToDoItem, complete_to_do_item)]
async fn complete_to_do_item(todo_id: i32) -> Result<Todo, NanoServiceError> {
    let query = r#"
        UPDATE todos
        SET finished = true, date_finished = NOW()
        WHERE id = $1
        RETURNING id, name, due_date, assigned_by, assigned_to, description, date_assigned, date_finished, finished
    "#;

    sqlx::query_as::<_, Todo>(query)
        .bind(todo_id)
        .fetch_one(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(format!("Failed to complete to-do item: {}", e), NanoServiceErrorStatus::Unknown))
}
