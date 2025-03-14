use dal::users::tx_definitions::DeleteUser;
use auth_core::api::users::delete_user::delete_user as delete_user_core;
use actix_web::{
    HttpResponse,
    web::Json
};
use utils::api_endpoint;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct DeleteUserBody {
    pub id: i32
}

#[api_endpoint(
    token=SuperAdminRoleCheck, 
    db_traits=[DeleteUser], 
)]
pub async fn delete_user(body: Json<DeleteUserBody>) {
    let _ = delete_user_core::<X>(body.id).await?;
    Ok(HttpResponse::Created().finish())
}