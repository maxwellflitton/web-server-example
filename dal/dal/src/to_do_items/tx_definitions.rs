//! Defines transaction traits for interacting with the `Todo` database table.
//!
//! # Overview
//! This file uses the `define_dal_transactions` macro to create traits for database transactions
//! specific to the `Todo` entities. Each trait represents a distinct database operation such as
//! creating, updating, retrieving, and deleting to-do items.
//!
//! ## Purpose
//! - Provide an interface for core logic to interact with the data access layer (DAL).
//! - Support dependency injection for database transaction implementations.
//!
//! ## Notes
//! - These traits are designed to be implemented by database descriptor structs, such as `SqlxPostGresDescriptor`.
//! - Adding a new database backend requires implementing these traits for the corresponding descriptor.
use kernel::to_do_items::{NewTodo, Todo};
use crate::define_dal_transactions;


define_dal_transactions!(
    CreateToDoItem => create_to_do_item(todo: NewTodo) -> Todo,
    DeleteToDoItem => delete_to_do_item(id: i32) -> bool,
    GetToDoItemsForUser => get_to_do_items_for_user(user_id: i32) -> Vec<Todo>,
    GetPendingToDoItemsForUser => get_pending_to_do_items_for_user(user_id: i32) -> Vec<Todo>,
    ReAssignToDoItem => re_assign_to_do_item(todo_id: i32, new_assigned_to: i32) -> Todo,
    CompleteToDoItem => complete_to_do_item(todo_id: i32) -> Todo
);
