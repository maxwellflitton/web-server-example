use actix_web::HttpResponse;
use kernel::token::session_cache::structs::IntoAuthCacheKey;
use utils::config::GetConfigVariable;
use kernel::token::session_cache::traits::DelAuthCacheSession;
use kernel::token::token::HeaderToken;
use kernel::token::checks::NoRoleCheck;

use utils::errors::NanoServiceError;


pub async fn logout<X, Y>(token: HeaderToken<Y, NoRoleCheck>) -> Result<HttpResponse, NanoServiceError> 
where
    X: DelAuthCacheSession,
    Y: GetConfigVariable
{
    X::del_auth_cache_session(token.into_auth_cache_key().key).await?;
    Ok(HttpResponse::Ok().finish())
}

