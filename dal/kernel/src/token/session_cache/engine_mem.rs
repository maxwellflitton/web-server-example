use crate::token::session_cache::traits::{GetAuthCacheSession, SetAuthCacheSession};
use crate::token::session_cache::structs::{AuthCacheSession, IntoAuthCacheKey, IntoAuthCacheSession};
use utils::errors::NanoServiceError;
use std::future::Future;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::LazyLock;

use super::traits::DelAuthCacheSession;


pub static SESSION_CACHE: LazyLock<Arc<Mutex<HashMap<String, AuthCacheSession>>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});


pub struct AuthCacheSessionEngineMem;


impl GetAuthCacheSession for AuthCacheSessionEngineMem {
    fn get_auth_cache_session<X: IntoAuthCacheKey + Send>(key: &X) 
    -> impl Future<Output = Result<Option<AuthCacheSession>, NanoServiceError>> + Send {
        let key = key.into_auth_cache_key();
        async move {
            let session = SESSION_CACHE.lock().await;
            match session.get(&key.key) {
                Some(session) => {
                    let session_ref = session.clone();
                    Ok(Some(session_ref))
                },
                None => Ok(None)
            }
        }
    }
}


impl SetAuthCacheSession for AuthCacheSessionEngineMem {
    fn set_auth_cache_session<X: IntoAuthCacheKey, Y: IntoAuthCacheSession>(key: &X, session: &Y) 
    -> impl Future<Output = Result<(), NanoServiceError>> + Send {
        let session = session.into_auth_cache_session();
        let key = key.into_auth_cache_key();
        async move {
            let mut session_cache = SESSION_CACHE.lock().await;
            session_cache.insert(key.key, session);
            Ok(())
        }
    }
}


impl DelAuthCacheSession for AuthCacheSessionEngineMem {

    fn del_auth_cache_session<X: IntoAuthCacheKey>(key: X) 
        -> impl Future<Output = Result<(), NanoServiceError>> + Send {
        let key = key.into_auth_cache_key();
        async move {
            let mut session_cache = SESSION_CACHE.lock().await;
            session_cache.remove(&key.key);
            Ok(())
        }
    }

}
