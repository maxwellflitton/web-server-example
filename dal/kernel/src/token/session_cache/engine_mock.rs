use crate::token::session_cache::traits::{GetAuthCacheSession, SetAuthCacheSession};
use crate::token::session_cache::structs::{AuthCacheSession, IntoAuthCacheKey, IntoAuthCacheSession};
use utils::errors::NanoServiceError;
use std::future::Future;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::LazyLock;
use crate::users::UserRole;
use chrono::Utc;


pub static SESSION_CACHE: LazyLock<Arc<Mutex<HashMap<String, AuthCacheSession>>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});


pub struct PassAuthSessionCheckMock;


impl GetAuthCacheSession for PassAuthSessionCheckMock {
    fn get_auth_cache_session<X: IntoAuthCacheKey + Send>(key: &X) 
    -> impl Future<Output = Result<Option<AuthCacheSession>, NanoServiceError>> + Send {
        let _key = key.into_auth_cache_key();
        async move {
            Ok(Some(AuthCacheSession{
                user_id: 1,
                role: UserRole::Admin,
                time_started: Utc::now(),
                time_expire: Utc::now(),
                user_agent: "test".to_string()
            }))
        }
    }
}


impl SetAuthCacheSession for PassAuthSessionCheckMock {
    fn set_auth_cache_session<X: IntoAuthCacheKey, Y: IntoAuthCacheSession>(_key: &X, _session: &Y) 
    -> impl Future<Output = Result<(), NanoServiceError>> + Send {
        async move {
            Ok(())
        }
    }
}


pub struct FailAuthSessionCheckMock;


impl GetAuthCacheSession for FailAuthSessionCheckMock {
    fn get_auth_cache_session<X: IntoAuthCacheKey + Send>(key: &X) 
    -> impl Future<Output = Result<Option<AuthCacheSession>, NanoServiceError>> + Send {
        let _key = key.into_auth_cache_key();
        async move {
            Ok(Some(AuthCacheSession{
                user_id: 1,
                role: UserRole::Admin,
                time_started: Utc::now(),
                time_expire: Utc::now(),
                user_agent: "test".to_string()
            }))
        }
    }
}