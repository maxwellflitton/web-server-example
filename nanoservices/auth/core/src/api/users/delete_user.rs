use utils::errors::NanoServiceError;
use dal::users::tx_definitions::DeleteUser;


pub async fn delete_user<X: DeleteUser>(id: i32) -> Result<bool, NanoServiceError> {
    X::delete_user(id).await
}
