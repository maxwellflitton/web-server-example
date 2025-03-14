use kernel::users::UserRole;
use dal::users::tx_definitions::GetUserByUuid;
use dal::role_permissions::tx_definitions::GetRolePermissions;
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use utils::config::GetConfigVariable;
use kernel::token::token::HeaderToken;
use kernel::token::checks::NoRoleCheck;
use kernel::token::session_cache::traits::{SetAuthCacheSession, DelAuthCacheSession};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct LoginReturnSchema {
    pub token: String,
    pub role: UserRole,
}



pub async fn refresh_token<X, Y, Z>(uuid: String, role: UserRole, user_agent: String) -> Result<LoginReturnSchema, NanoServiceError> 
where
    X: GetUserByUuid + GetRolePermissions,
    Y: GetConfigVariable,
    Z: SetAuthCacheSession + DelAuthCacheSession
{
    // Retrieve user information from the database
    let user = X::get_user_by_uuid(uuid.clone()).await?;

    if user.blocked {
        return Err(NanoServiceError::new(
            "User is blocked".to_string(), 
            NanoServiceErrorStatus::Unauthorized
        ));
    }
    if user.confirmed == false {
        return Err(NanoServiceError::new(
            "User is not confirmed".to_string(), 
            NanoServiceErrorStatus::Unauthorized
        ));
    }
    
    // Retrieve the roles associated with the user
    let roles: Vec<UserRole> = X::get_role_permissions(user.id).await?.into_iter().map(|r| r.role).collect();
    
    // Check if the user has the required role
    if !roles.contains(&role) {
        return Err(NanoServiceError::new(
            "User does not have the required role".to_string(), 
            NanoServiceErrorStatus::Unauthorized
        ));
    }
    
    // Generate authentication token
    let token: HeaderToken<Y, NoRoleCheck> = HeaderToken::new(user_agent, user.id, role.clone());
    
    // save to the cache session
    let _ = Z::del_auth_cache_session(uuid).await?;
    let _ = Z::set_auth_cache_session(&token, &token).await?;
    Ok(LoginReturnSchema { 
        token: token.encode()?,
        role: role
    })
}