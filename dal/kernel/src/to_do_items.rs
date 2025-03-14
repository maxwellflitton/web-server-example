//! Defines the `NewTodo` and `Todo` structs for managing to-do items in the system.
//!
//! # Purpose
//! - Enable database interactions through `Todo` and `NewTodo` structs.
//! - Support service-level operations and data transfers related to to-do tasks.
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

/// Represents the schema for creating a new to-do item.
///
/// # Fields
/// * `name`: The name or title of the task.
/// * `due_date`: The due date of the task (optional).
/// * `assigned_by`: The ID of the user who assigned the task.
/// * `assigned_to`: The ID of the user to whom the task is assigned.
/// * `description`: A detailed description of the task.
/// * `date_assigned`: The timestamp of when the task was assigned (optional).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewTodo {
    pub name: String,
    pub due_date: Option<NaiveDateTime>,
    pub assigned_by: i32,
    pub assigned_to: i32,
    pub description: Option<String>,
    pub date_assigned: Option<NaiveDateTime>,
}

/// Represents a to-do item retrieved from the database.
///
/// # Fields
/// * `id`: The unique identifier of the to-do item.
/// * `name`: The name or title of the task.
/// * `due_date`: The due date of the task (optional).
/// * `assigned_by`: The ID of the user who assigned the task.
/// * `assigned_to`: The ID of the user to whom the task is assigned.
/// * `description`: A detailed description of the task.
/// * `date_assigned`: The timestamp of when the task was assigned.
/// * `date_finished`: The timestamp of when the task was finished (optional).
/// * `finished`: Whether the task is marked as finished.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub due_date: Option<NaiveDateTime>,
    pub assigned_by: i32,
    pub assigned_to: i32,
    pub description: Option<String>,
    pub date_assigned: NaiveDateTime,
    pub date_finished: Option<NaiveDateTime>,
    pub finished: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    /// Tests creating a new `NewTodo` instance and ensuring fields are set correctly.
    #[test]
    fn test_create_new_todo() {
        let name = "Test Task".to_string();
        let due_date = Some(Utc::now().naive_utc());
        let assigned_by = 1;
        let assigned_to = 2;
        let description = Some("This is a test task".to_string());
        let date_assigned = Some(Utc::now().naive_utc());

        let new_todo = NewTodo {
            name: name.clone(),
            due_date,
            assigned_by,
            assigned_to,
            description: description.clone(),
            date_assigned,
        };

        assert_eq!(new_todo.name, name);
        assert_eq!(new_todo.assigned_by, assigned_by);
        assert_eq!(new_todo.assigned_to, assigned_to);
        assert_eq!(new_todo.description, description);
    }

    /// Tests creating a `Todo` instance and verifying its contents.
    #[test]
    fn test_todo_struct() {
        let now = Utc::now().naive_utc();
        let todo = Todo {
            id: 1,
            name: "Task 1".to_string(),
            due_date: Some(now),
            assigned_by: 1,
            assigned_to: 2,
            description: Some("Complete this task".to_string()),
            date_assigned: now,
            date_finished: None,
            finished: false,
        };

        assert_eq!(todo.id, 1);
        assert_eq!(todo.finished, false);
        assert_eq!(todo.name, "Task 1");
    }
}
