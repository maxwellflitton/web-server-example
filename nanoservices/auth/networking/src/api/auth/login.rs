// External crates
use actix_web::{HttpResponse, HttpRequest, web::Json};
use crate::utils::extract_basic_auth_credentials;
use auth_core::api::auth::login::login as login_core;
use kernel::users::UserRole;
use serde::Deserialize;
use dal::users::tx_definitions::GetUserByEmail;
use dal::role_permissions::tx_definitions::GetRolePermissions;
use utils::config::GetConfigVariable;
use kernel::token::session_cache::traits::SetAuthCacheSession;

use utils::errors::{NanoServiceError, NanoServiceErrorStatus};


#[derive(Deserialize, Debug)]
pub struct LoginBody {
    pub role: UserRole
}


/// This endpoint logs the user in.
pub async fn login<X, Y, Z>(req: HttpRequest, body: Json<LoginBody>) -> Result<HttpResponse, NanoServiceError> 
where
    X: GetUserByEmail + GetRolePermissions,
    Y: GetConfigVariable,
    Z: SetAuthCacheSession,
{
    let (email, password) = extract_basic_auth_credentials(&req)?;
    let agent_value = match req.headers().get("User-Agent") {
        Some(value) => value,
        None => return Err(
            NanoServiceError::new("No User-Agent header found".to_string(), NanoServiceErrorStatus::Unauthorized)
        )
    };
    let agent_string = agent_value.to_str().map_err(|e| NanoServiceError::new(
        e.to_string(), NanoServiceErrorStatus::Unauthorized
    ))?.to_string();
    let login_response = match login_core::<X, Y, Z>(email, password, body.into_inner().role, agent_string).await {
        Ok(login_response) => login_response,
        Err(e) => {
            return Err(e)
        }
    };
    Ok(HttpResponse::Ok().json(login_response))
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header, http::header::HeaderValue};
    use actix_web::{
        dev::ServiceResponse,
        self, body::MessageBody, http::header::ContentType, test::{
            call_service, init_service, TestRequest
        }, web, App
    };
    use actix_http::Request;
    use dal_tx_impl::impl_transaction;
    use base64::{Engine as _, engine::general_purpose};
    use kernel::role_permissions::RolePermission;
    use kernel::users::{User, NewUser};
    use serde_json::json;
    use kernel::token::session_cache::engine_mock::PassAuthSessionCheckMock;
    use auth_core::api::auth::login::LoginReturnSchema;

    fn generate_user(password: String, user_role: UserRole) -> User {
        let new_user = NewUser::new(
            "test_username".to_string(),
            "test@gmail.com".to_string(),
            "first_name".to_string(),
            "last_name".to_string(),
            user_role,
            password
        ).unwrap();
        User {
            id: 1,
            confirmed: true,
            username: new_user.username,
            email: new_user.email,
            password: new_user.password,
            first_name: new_user.first_name,
            last_name: new_user.last_name,
            user_role: new_user.user_role,
            date_created: new_user.date_created,
            last_logged_in: new_user.last_logged_in,
            blocked: new_user.blocked,
            uuid: new_user.uuid,
        }
    }

    #[tokio::test]
    async fn test_pass() {

        struct MockPostgres;
        struct MockConfig;

        #[impl_transaction(MockPostgres, GetUserByEmail, get_user_by_email)]
        async fn get_user_by_email(email: String) -> Result<User, NanoServiceError> {
            assert_eq!(email, "test@gmail.com".to_string());
            Ok(generate_user("password".to_string(), UserRole::Admin))
        }

        #[impl_transaction(MockPostgres, GetRolePermissions, get_role_permissions)]
        async fn get_role_permissions(user_id: i32) -> Result<Vec<RolePermission>, NanoServiceError> {
            assert_eq!(user_id, 1);
            Ok(vec![RolePermission {
                id: 1,
                user_id: 1,
                role: UserRole::Admin,
            }])
        }
        impl GetConfigVariable for MockConfig {
            fn get_config_variable(_key: String) -> Result<String, NanoServiceError> {
                Ok("secret".to_string())
            }
        }

        async fn run_request(req: Request) -> ServiceResponse {
            let service = login::<MockPostgres, MockConfig, PassAuthSessionCheckMock>;
            let app = init_service(App::new().route("/login", web::post().to(service))).await;
            call_service(&app, req).await
        }

        let credentials = "test@gmail.com:password";
        let encoded_credentials = general_purpose::STANDARD.encode(credentials);
        let auth_header_value = HeaderValue::from_str(&format!("Basic {}", encoded_credentials)).unwrap();
        let body = json!({
            "role": "AdMiN"
        });
        let req = TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header((header::AUTHORIZATION, auth_header_value))
            .insert_header((header::USER_AGENT, "some-agent"))
            .uri("/login")
            .set_json(&body)
            .to_request();
        let resp = run_request(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();
        let _response_body: LoginReturnSchema = serde_json::from_str(body_str).unwrap();

        assert_eq!(status, 200);
    }
}