use dal::to_do_items::tx_definitions::{CreateToDoItem, GetToDoItemsForUser};
use kernel::to_do_items::NewTodo;
use utils::api_endpoint;
use actix_web::{
    HttpResponse,
    web::Json
};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct NewTodoBody {
    pub item: NewTodo,
    pub user_id: i32
}


#[api_endpoint(token=AdminRoleCheck, db_traits=[CreateToDoItem, GetToDoItemsForUser], env_variable_trait=true)]
pub async fn create_to_do_item(new_todo: Json<NewTodo>) {
    let new_item = new_todo.into_inner();
    let user_id = new_item.assigned_to;
    let _ = X::create_to_do_item(new_item).await?;
    let items = X::get_to_do_items_for_user(user_id).await?;
    Ok(HttpResponse::Created().json(items))
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernel::users::UserRole;
    use dal_tx_impl::impl_transaction;
    use utils::errors::NanoServiceError;
    use kernel::token::session_cache::engine_mock::PassAuthSessionCheckMock;
    use kernel::token::checks::SuperAdminRoleCheck;
    use utils::send_test_request;
    use kernel::to_do_items::Todo;
    use chrono::Utc;

    #[tokio::test]
    async fn test_create_item() {
        struct MockPostgres;

        #[impl_transaction(MockPostgres, CreateToDoItem, create_to_do_item)]
        async fn create_to_do_item(todo: NewTodo) -> Result<Todo, NanoServiceError> {
            let now = Utc::now().naive_utc();

            Ok(Todo {
                id: 1,                                // Mock ID
                name: todo.name.clone(),              // Name from input
                due_date: todo.due_date,              // Optional due date from input
                assigned_by: todo.assigned_by,        // Assigned by from input
                assigned_to: todo.assigned_to,        // Assigned to from input
                description: todo.description.clone(),// Optional description from input
                date_assigned: todo.date_assigned.unwrap_or(now), // Use input or current timestamp
                date_finished: None,                  // Not finished on creation
                finished: false,                      // Not finished on creation
            })
        }


        #[impl_transaction(MockPostgres, GetToDoItemsForUser, get_to_do_items_for_user)]
        async fn get_to_do_items_for_user(user_id: i32) -> Result<Vec<Todo>, NanoServiceError> {
            let now = Utc::now().naive_utc();

            let todos = (1..=5).map(|i| {
                Todo {
                    id: i,
                    name: format!("Mock Task {}", i),
                    due_date: Some(now + chrono::Duration::days(i.into())),
                    assigned_by: 100, // Mock assigner user id
                    assigned_to: user_id,
                    description: Some(format!("Description for task {}", i)),
                    date_assigned: now,
                    date_finished: None,
                    finished: false,
                }
            }).collect();

            Ok(todos)
        }

        send_test_request!(
            POST, 
            "/create", 
            serde_json::json!({
                "name": "Test To-Do Item",
                "due_date": Utc::now().naive_utc(),
                "assigned_by": 1,
                "assigned_to": 2,
                "description": "This is a test description",
                "date_assigned": Utc::now().naive_utc()
            }),
            SuperAdminRoleCheck,
            UserRole::SuperAdmin,
            1,
            create_to_do_item,
            MockPostgres, MockConfig, PassAuthSessionCheckMock
        );

        let resp = send_request().await;
        assert_eq!(resp.status(), 201);
    }

}
