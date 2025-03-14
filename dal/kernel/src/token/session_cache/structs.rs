use crate::users::UserRole;
use chrono::{DateTime, Utc};


#[derive(Debug, Clone)]
pub struct AuthCacheSession {
    pub user_id: i32,
    pub role: UserRole,
    pub time_started: DateTime<Utc>,
    pub time_expire: DateTime<Utc>,
    pub user_agent: String,
}


pub struct AuthCacheKey {
    pub key: String
}


pub trait IntoAuthCacheSession {
    fn into_auth_cache_session(&self) -> AuthCacheSession;
}

pub trait IntoAuthCacheKey {
    fn into_auth_cache_key(&self) -> AuthCacheKey;
}

impl IntoAuthCacheKey for String {
    fn into_auth_cache_key(&self) -> AuthCacheKey {
        AuthCacheKey {
            key: self.clone()
        }
    }
}

impl IntoAuthCacheKey for &str {
    fn into_auth_cache_key(&self) -> AuthCacheKey {
        AuthCacheKey {
            key: self.to_string()
        }
    }
}
