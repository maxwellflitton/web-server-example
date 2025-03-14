use actix_web::{web, HttpResponse};
use auth_core::api::users::update::update_user_fields;
use utils::api_endpoint;
use serde::{Serialize, Deserialize};
use dal::users::tx_definitions::{
    UpdateUserUsername,
    UpdateUserEmail,
    UpdateUserFirstName,
    UpdateUserLasttName,
    GetUser
};

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateUserBody {
    pub username: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub id: i32
}


#[api_endpoint(
    token=SuperAdminRoleCheck, 
    db_traits=[UpdateUserUsername, UpdateUserEmail, UpdateUserFirstName, UpdateUserLasttName, GetUser]
)]
pub async fn update(body: web::Json<UpdateUserBody>)  {
    let body: UpdateUserBody = body.into_inner();
    let updated_user = update_user_fields::<X>(
        body.id, 
        body.username, 
        body.email, 
        body.first_name, 
        body.last_name
    ).await?;
    Ok(HttpResponse::Ok().json(updated_user))
}
