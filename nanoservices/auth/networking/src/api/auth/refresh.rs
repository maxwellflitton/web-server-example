// External crates
use actix_web::HttpResponse;
use auth_core::api::auth::refresh::refresh_token;
use dal::role_permissions::tx_definitions::GetRolePermissions;
use dal::users::tx_definitions::GetUserByUuid;
use utils::config::GetConfigVariable;
use kernel::token::session_cache::traits::{SetAuthCacheSession, DelAuthCacheSession};
use kernel::token::checks::NoRoleCheck;
use kernel::token::token::HeaderToken;

use utils::errors::NanoServiceError;


pub async fn refresh<X, Y, Z>(token: HeaderToken<Y, NoRoleCheck>) -> Result<HttpResponse, NanoServiceError> 
where
    X: GetUserByUuid + GetRolePermissions,
    Y: GetConfigVariable,
    Z: SetAuthCacheSession + DelAuthCacheSession,
{
    let login_response = match refresh_token::<X, Y, Z>(
        token.unique_id.clone(), token.role, token.user_agent).await {
        Ok(login_response) => login_response,
        Err(e) => {
            return Err(e)
        }
    };
    Ok(HttpResponse::Ok().json(login_response))
}