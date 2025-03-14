use crate::token::session_cache::structs::{AuthCacheSession, IntoAuthCacheKey, IntoAuthCacheSession};
use utils::errors::NanoServiceError;
use std::future::Future;


pub trait GetAuthCacheSession {
    fn get_auth_cache_session<X: IntoAuthCacheKey + Send>(key: &X) 
    -> impl Future<Output = Result<Option<AuthCacheSession>, NanoServiceError>> + Send;
}

pub trait SetAuthCacheSession {
    fn set_auth_cache_session<X: IntoAuthCacheKey, Y: IntoAuthCacheSession>(key: &X, session: &Y) 
    -> impl Future<Output = Result<(), NanoServiceError>> + Send;
}

pub trait DelAuthCacheSession {
    fn del_auth_cache_session<X: IntoAuthCacheKey>(key: X) 
    -> impl Future<Output = Result<(), NanoServiceError>> + Send;
}
