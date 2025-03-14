use utils::errors::NanoServiceError;
use dal::users::tx_definitions::{
    UpdateUserUsername,
    UpdateUserEmail,
    UpdateUserFirstName,
    UpdateUserLasttName,
    GetUser
};
use kernel::users::User;

/// Updates a user’s fields if provided.
///
/// # Arguments
/// - `id`: User ID.
/// - `username`: Optional username update.
/// - `email`: Optional email update.
/// - `first_name`: Optional first name update.
/// - `last_name`: Optional last name update.
///
/// # Returns
/// - `Ok(User)`: The updated user.
/// - `Err(NanoServiceError)`: If an error occurs.
pub async fn update_user_fields<X>(
    id: i32,
    username: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>
) -> Result<User, NanoServiceError> 
where
    X: UpdateUserEmail + UpdateUserFirstName + UpdateUserLasttName + UpdateUserUsername + GetUser
{
    match username {
        Some(username) => {X::update_user_username(id, username).await?; ()},
        None => ()
    }
    match email {
        Some(email) => {X::update_user_email(id, email).await?; ()},
        None => ()
    }
    match first_name {
        Some(first_name) => {X::update_user_first_name(id, first_name).await?; ()},
        None => ()
    }
    match last_name {
        Some(last_name) => {X::update_user_last_name(id, last_name).await?; ()},
        None => ()
    }
    X::get_user(id).await
}
