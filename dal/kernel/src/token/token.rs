//! This module houses the token implementation for JWT
// External crate imports
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use chrono::{DateTime, Utc};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{
    Algorithm, 
    DecodingKey, 
    EncodingKey, 
    Header, 
    Validation, 
    decode, 
    encode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::marker::PhantomData;

// Local crate imports
use crate::token::checks::CheckUserRole;
use crate::users::UserRole;
use utils::{
    config::GetConfigVariable,
    errors::{NanoServiceError, NanoServiceErrorStatus},
};
use crate::token::session_cache::{
    structs::{IntoAuthCacheSession, AuthCacheSession, IntoAuthCacheKey, AuthCacheKey},
    traits::GetAuthCacheSession
};
use std::future::Future;


/// The auth token extracted from the header for logged in users.
/// 
/// # Fields
/// * `unique_id` - The unique id of the token for the auth session
/// * `user_id` - The id of the user
/// * `role` - The role of the user
/// * `time_started` - The time the token was created
/// * `time_expire` - The time the token will expire
/// * `user_agent` - The device info of the user
#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderToken<X: GetConfigVariable, Y: CheckUserRole> {
    pub unique_id: String,
    pub user_id: i32,
    pub role: UserRole,
    pub time_started: DateTime<Utc>,
    pub time_expire: DateTime<Utc>,
    pub user_agent: String,
    pub var_handle: PhantomData<X>,
    pub role_handle: PhantomData<Y>
}


impl<X: GetConfigVariable, Y: CheckUserRole> IntoAuthCacheSession for HeaderToken<X, Y> {
    fn into_auth_cache_session(&self) -> AuthCacheSession {
        AuthCacheSession {
            user_id: self.user_id,
            role: self.role.clone(),
            time_started: self.time_started,
            time_expire: self.time_expire,
            user_agent: self.user_agent.clone()
        }
    }
}

impl<X: GetConfigVariable, Y: CheckUserRole> IntoAuthCacheKey for HeaderToken<X, Y> {
    fn into_auth_cache_key(&self) -> AuthCacheKey {
        AuthCacheKey {
            key: self.unique_id.clone()
        }
    }
}


impl <X: GetConfigVariable, Y: CheckUserRole>HeaderToken<X, Y> {

    /// Creates a new token for a user.
    /// 
    /// # Arguments
    /// * `user_agent` - The device info of the user
    /// * `user_id` - The id of the user
    /// * `user_role` - The role of the user
    /// 
    /// # Returns
    /// * A new token for the user
    pub fn new(user_agent: String, user_id: i32, user_role: UserRole) -> Self {
        HeaderToken {
            unique_id: Uuid::new_v4().to_string(),
            user_id: user_id,
            role: user_role,
            time_started: Utc::now(),
            time_expire: Utc::now() + chrono::Duration::minutes(20),
            user_agent: user_agent,
            var_handle: PhantomData,
            role_handle: PhantomData
        }
    }

    /// Checks the device info in the request to see if it matches the device info in the token.
    /// 
    /// # Arguments
    /// * `req` - The request to check the device info of
    /// 
    /// # Returns
    /// * `Result<(), NanoServiceError>` - The result of the check
    pub fn check_device_info(&self, req: &HttpRequest) -> Result<(), NanoServiceError> {
        let user_agent_str = req
            .headers()
            .get("User-Agent")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown");

        if user_agent_str != self.user_agent {
            return Err(
                NanoServiceError::new(
                    "User-Agent does not match".to_string(),
                    NanoServiceErrorStatus::Unauthorized
                )
            )
        }
        Ok(())
    }

    /// Checks if the token has expired.
    /// 
    /// # Returns
    /// * error if the token has expired
    pub fn check_if_expired(&self) -> Result<(), NanoServiceError> {
        let now = Utc::now();
        if now > self.time_expire {
            return Err(
                NanoServiceError::new(
                    "Token has expired".to_string(),
                    NanoServiceErrorStatus::Unauthorized
                )
            )
        }
        Ok(())
    }

    /// Encodes the struct into a token.
    ///
    /// # Returns
    /// encoded token with fields of the current struct
    pub fn encode(self) -> Result<String, NanoServiceError> {
        let key_str = <X>::get_config_variable("SECRET_KEY".to_string())?;
        let key = EncodingKey::from_secret(key_str.as_ref());
        return match encode(&Header::default(), &self, &key) {
            Ok(token) => Ok(token),
            Err(error) => Err(
                NanoServiceError::new(
                    error.to_string(),
                    NanoServiceErrorStatus::Unauthorized
                )
            )
        };
    }

    /// Decodes the token into a struct.
    ///
    /// # Arguments
    /// * `token` - The token to be decoded.
    ///
    /// # Returns
    /// decoded token with fields of the current struct
    pub fn decode(token: &str) -> Result<Self, NanoServiceError> {
        let key_str = <X>::get_config_variable("SECRET_KEY".to_string())?;
        let key = DecodingKey::from_secret(key_str.as_ref());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.required_spec_claims.remove("exp");

        match decode::<Self>(token, &key, &validation) {
            Ok(token_data) => return Ok(token_data.claims),
            Err(error) => return Err(
                NanoServiceError::new(
                    error.to_string(),
                    NanoServiceErrorStatus::Unauthorized
                )
            )
        };
    }

    /// Gets the session cache via the token's unique id.
    /// 
    /// # Returns
    /// * The session cache related to the token
    pub fn get_in_session_cache<C: GetAuthCacheSession>(&self) 
    -> impl Future<Output = Result<Option<AuthCacheSession>, NanoServiceError>> {
        let key = self.unique_id.clone();
        async move {
            C::get_auth_cache_session(&key).await
        }
    } 
}


impl<X: GetConfigVariable, Y: CheckUserRole> FromRequest for HeaderToken<X, Y> {
    type Error = NanoServiceError;
    type Future = Ready<Result<HeaderToken<X, Y>, NanoServiceError>>;

    /// This function fires before the API request function is loaded.
    /// 
    /// # Arguments
    /// * `req` - The request to extract the token from
    /// 
    /// # Returns
    /// * The token or an unauthorized error which is directly returned to the user
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // extract the token from the header
        let raw_data = match req.headers().get("token") {
            Some(data) => data,
            None => {
                return err(NanoServiceError {
                    status: NanoServiceErrorStatus::Unauthorized,
                    message: "token not in header under key 'token'".to_string()
                })
            }
        };
        // convert the token to a string
        let message = match raw_data.to_str() {
            Ok(token) => token.to_string(),
            Err(_) => {
                return err(NanoServiceError {
                    status: NanoServiceErrorStatus::Unauthorized,
                    message: "token not a valid string".to_string()
                })
            }
        };
        // decode the token and perform role and device checks
        let token = match HeaderToken::decode(&message) {
            Ok(token) => {
                let unwrapped_token = token;
                match unwrapped_token.check_device_info(&req) {
                    Ok(_) => (),
                    Err(e) => {
                        return err(e)
                    }
                };
                match Y::check_user_role(&unwrapped_token.role) {
                    Ok(_) => (),
                    Err(e) => {
                        return err(e)
                    }
                }
                // check if the token has expired
                let now = Utc::now();
                if now > unwrapped_token.time_expire {
                    return err(
                        NanoServiceError::new(
                            "Token has expired".to_string(),
                            NanoServiceErrorStatus::Unauthorized
                        )
                    )
                }
                unwrapped_token
            },
            Err(e) => {
                return err(e)
            }
        };


        return ok(token)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;
    use actix_web::{
        self, body::MessageBody, http::header::ContentType, test::{
            call_service, init_service, TestRequest
        }, web, App, HttpRequest, HttpResponse
    };
    use utils::errors::NanoServiceError;
    use crate::token::checks::{
        NoRoleCheck,
        AdminRoleCheck, 
        SuperAdminRoleCheck,
        WorkerRoleCheck,
        ExactAdminRoleCheck
    };

    static USER_AGENT : &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3";
    static DIFFERENT_USER_AGENT : &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:89.0) Gecko/20100101 Firefox/89.0";
    struct FakeConfig;

    impl GetConfigVariable for FakeConfig {

        fn get_config_variable(variable: String) -> Result<String, NanoServiceError> {
            match variable.as_str() {
                "SECRET_KEY" => Ok("secret".to_string()),
                _ => Ok("".to_string())
            }
        }

    }

    async fn pass_handle(token: HeaderToken<FakeConfig, NoRoleCheck>, _: HttpRequest) -> HttpResponse {
        return HttpResponse::Ok().json(json!({"user_id": token.user_id}))
    }

    async fn super_admin_handle(token: HeaderToken<FakeConfig, SuperAdminRoleCheck>, _: HttpRequest) -> HttpResponse {
        return HttpResponse::Ok().json(json!({"user_id": token.user_id}))
    }

    async fn admin_handle(token: HeaderToken<FakeConfig, AdminRoleCheck>, _: HttpRequest) -> HttpResponse {
        return HttpResponse::Ok().json(json!({"user_id": token.user_id}))
    }

    async fn worker_handle(token: HeaderToken<FakeConfig, WorkerRoleCheck>, _: HttpRequest) -> HttpResponse {
        return HttpResponse::Ok().json(json!({"user_id": token.user_id}))
    }

    async fn exact_check_handle(token: HeaderToken<FakeConfig, ExactAdminRoleCheck>, _: HttpRequest) -> HttpResponse {
        return HttpResponse::Ok().json(json!({"user_id": token.user_id}))
    }

    /// Because it's being constructed to be sent it doesn't matter what role check is used
    fn construct_token(user_role: UserRole) -> HeaderToken<FakeConfig, NoRoleCheck> {
        HeaderToken::new(USER_AGENT.to_string(), 1, user_role)
    }

    #[test]
    fn test_encode_decode() {
        let jwt = construct_token(UserRole::Admin);
        let _ = jwt.encode().unwrap();
    }

    #[test]
    fn test_decode_token() {
        let expected_token = construct_token(UserRole::Admin).encode().unwrap();
        let decoded_token = HeaderToken::<FakeConfig, NoRoleCheck>::decode(&expected_token).unwrap();
        assert_eq!(decoded_token.user_id, 1);
    }

    #[actix_web::test]
    async fn test_fail_no_token_role_check() {
        let app = init_service(App::new().route("/", web::get().to(pass_handle))).await;
        let req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();

        let resp = call_service(&app, req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(401, status);
        assert_eq!("\"token not in header under key 'token'\"", body_str);
    }

    #[actix_web::test]
    async fn test_fail_no_agent_role_check() {
        let app = init_service(App::new().route("/", web::get().to(pass_handle))).await;
        let req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .insert_header(("token", construct_token(UserRole::Admin).encode().unwrap()))
            .to_request();

        let resp = call_service(&app, req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(401, status);
        assert_eq!("\"User-Agent does not match\"", body_str);
    }

    #[actix_web::test]
    async fn test_fail_wrong_agent_role_check() {
        let app = init_service(App::new().route("/", web::get().to(pass_handle))).await;
        let req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .insert_header(("token", construct_token(UserRole::Admin).encode().unwrap()))
            .insert_header(("User-Agent", DIFFERENT_USER_AGENT))
            .to_request();

        let resp = call_service(&app, req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(401, status);
        assert_eq!("\"User-Agent does not match\"", body_str);
    }

    #[actix_web::test]
    async fn test_pass_no_role_check() {

        let app = init_service(App::new().route("/", web::get().to(pass_handle))).await;
        let req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .insert_header(("token", construct_token(UserRole::Admin).encode().unwrap()))
            .insert_header(("User-Agent", USER_AGENT))
            .to_request();

        let resp = call_service(&app, req).await;
        assert_eq!("200", resp.status().as_str());
    }

    #[actix_web::test]
    async fn test_fail_super_admin_check() {
        let app = init_service(App::new().route("/", web::get().to(super_admin_handle))).await;
        let req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .insert_header(("token", construct_token(UserRole::Admin).encode().unwrap()))
            .insert_header(("User-Agent", USER_AGENT))
            .to_request();

        let resp = call_service(&app, req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(401, status);
        assert_eq!("\"Role does not have sufficient permissions\"", body_str);
    }

    #[actix_web::test]
    async fn test_pass_admin_check() {
        let app = init_service(App::new().route("/", web::get().to(admin_handle))).await;
        let req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .insert_header(("token", construct_token(UserRole::Admin).encode().unwrap()))
            .insert_header(("User-Agent", USER_AGENT))
            .to_request();

        let resp = call_service(&app, req).await;
        assert_eq!("200", resp.status().as_str());
    }

    #[actix_web::test]
    async fn test_pass_worker_check() {
        let app = init_service(App::new().route("/", web::get().to(worker_handle))).await;
        let req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .insert_header(("token", construct_token(UserRole::Admin).encode().unwrap()))
            .insert_header(("User-Agent", USER_AGENT))
            .to_request();

        let resp = call_service(&app, req).await;
        assert_eq!("200", resp.status().as_str());
    }

    #[actix_web::test]
    async fn test_pass_exact_check() {
        let app = init_service(App::new().route("/", web::get().to(exact_check_handle))).await;
        let req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .insert_header(("token", construct_token(UserRole::Admin).encode().unwrap()))
            .insert_header(("User-Agent", USER_AGENT))
            .to_request();

        let resp = call_service(&app, req).await;
        assert_eq!("200", resp.status().as_str());
    }

    #[actix_web::test]
    async fn test_fail_exact_low_check() {
        let app = init_service(App::new().route("/", web::get().to(exact_check_handle))).await;
        let req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .insert_header(("token", construct_token(UserRole::Worker).encode().unwrap()))
            .insert_header(("User-Agent", USER_AGENT))
            .to_request();

        let resp = call_service(&app, req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(401, status);
        assert_eq!("\"Role does not have sufficient permissions\"", body_str);
    }

    #[actix_web::test]
    async fn test_fail_exact_high_check() {
        let app = init_service(App::new().route("/", web::get().to(exact_check_handle))).await;
        let req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .insert_header(("token", construct_token(UserRole::SuperAdmin).encode().unwrap()))
            .insert_header(("User-Agent", USER_AGENT))
            .to_request();

        let resp = call_service(&app, req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(401, status);
        assert_eq!("\"Role does not have sufficient permissions\"", body_str);
    }

    #[actix_web::test]
    async fn test_fail_timeout() {
        let mut jwt = construct_token(UserRole::Admin);
        jwt.time_expire = Utc::now() - chrono::Duration::minutes(1);
        let app = init_service(App::new().route("/", web::get().to(pass_handle))).await;
        let req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .insert_header(("token", jwt.encode().unwrap()))
            .insert_header(("User-Agent", USER_AGENT))
            .to_request();

        let resp = call_service(&app, req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(401, status);
        assert_eq!("\"Token has expired\"", body_str);
    }

}