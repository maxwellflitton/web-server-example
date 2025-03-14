//! Networking layer for user retrieval operations.
//!
//! # Overview
//! This module provides Actix Web endpoints for retrieving users by:
//! - `id`
//! - `email`
//! - `uuid`
//!
//! # Features
//! - Uses generics to inject database operations via DAL traits.
//! - Converts errors to proper HTTP responses using `NanoServiceError`.
//!
//! # Routes
//! - `GET /api/auth/v1/users/{id}`
//! - `GET /api/auth/v1/users/by-email/{email}`
//! - `GET /api/auth/v1/users/by-uuid/{uuid}`

use actix_web::{web, HttpResponse};
use kernel::users::{TrimmedUser, UserRole};
use auth_core::api::users::get::{get_user, get_user_by_email, get_user_by_uuid};
use dal::users::tx_definitions::{GetUser, GetUserByEmail, GetUserByUuid};
use dal::role_permissions::tx_definitions::GetRolePermissions;
use serde::{Serialize, Deserialize};
use utils::api_endpoint;


/// Represents a user profile containing user details and roles.
/// 
/// # Notes
/// This is private and to wrap the returns of the roles
/// 
/// # Fields
/// - `user`: The user details.
/// - `roles`: The roles assigned to the user.
#[derive(Serialize, Deserialize)]
struct UserProfile {
    pub user: TrimmedUser,
    pub roles: Vec<UserRole>, 
}

/// gets the roles for the user and returns the profile as a HTTP response.
macro_rules! return_profile {
    ($id:expr, $user:ident) => {{
        let roles = X::get_role_permissions($id).await?;
        let roles: Vec<UserRole> = roles.into_iter().map(|role| role.role).collect();
        Ok(HttpResponse::Ok().json(UserProfile { user: $user, roles }))
    }};
}

#[api_endpoint(token=SuperAdminRoleCheck, db_traits=[GetUser, GetRolePermissions])]
pub async fn get_user_by_id(path: web::Path<i32>) {
    let id = path.into_inner();
    let user: TrimmedUser = get_user::<X>(id).await?.into();
    return_profile!(id, user)
}

#[api_endpoint(token=SuperAdminRoleCheck, db_traits=[GetUserByEmail, GetRolePermissions])]
pub async fn get_user_by_email_route(path: web::Path<String>) {
    let email = path.into_inner();
    let user: TrimmedUser = get_user_by_email::<X>(email).await?.into();
    return_profile!(user.id, user)
}

#[api_endpoint(db_traits=[GetUserByUuid, GetRolePermissions])]
pub async fn get_user_by_uuid_route(path: web::Path<String>) {
    let uuid = path.into_inner();
    let user: TrimmedUser = get_user_by_uuid::<X>(uuid).await?.into();
    return_profile!(user.id, user)
}

#[api_endpoint(token=NoRoleCheck, db_traits=[GetUser, GetRolePermissions])]
pub async fn get_by_jwt() {
    let user: TrimmedUser = X::get_user(jwt.user_id).await?.into();
    return_profile!(user.id, user)
}


#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::http::header;
    use actix_web::{
        dev::ServiceResponse,
        self, body::MessageBody, test::{
            call_service, init_service, TestRequest
        }, web, App
    };
    use actix_http::Request;
    use kernel::users::{User, NewUser};
    use dal_tx_impl::impl_transaction;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::LazyLock;
    use kernel::users::UserRole;
    use kernel::role_permissions::RolePermission;
    use utils::errors::NanoServiceError;
    use kernel::token::token::HeaderToken;
    use kernel::token::session_cache::engine_mock::PassAuthSessionCheckMock;
    use utils::config::GetConfigVariable;
    use kernel::token::checks::{SuperAdminRoleCheck, NoRoleCheck};

    struct MockConfig;

    impl GetConfigVariable for MockConfig {
        fn get_config_variable(_key: String) -> Result<String, NanoServiceError> {
            Ok("secret".to_string())
        }
    }

    fn generate_new_user(email: String, uuid: String) -> NewUser {
        NewUser {
            username: "test".to_string(),
            email: email,
            confirmed: true,
            password: "password".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            user_role: UserRole::Admin,
            uuid: uuid,
            blocked: false,
            last_logged_in: chrono::Utc::now().naive_utc(),
            date_created: chrono::Utc::now().naive_utc(),
        }
    }


    fn generate_user(user: NewUser, id: i32) -> User {
        let now = chrono::Utc::now().naive_utc();
        User {
            id: id,
            confirmed: false,
            username: user.username.clone(),
            email: user.email.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            user_role: user.user_role.clone(),
            password: user.password.clone(),
            uuid: user.uuid.clone(),
            date_created: now,
            last_logged_in: now,
            blocked: user.blocked,
        }
    }

    macro_rules! impl_roles {
        ($id:expr) => {
            #[impl_transaction(MockDbHandle, GetRolePermissions, get_role_permissions)]
            async fn get_role_permissions(user_id: i32) -> Result<Vec<RolePermission>, NanoServiceError> {
                GET_USER_PERMISSIONS.store(true, Ordering::Relaxed);
                assert_eq!(user_id, $id);
                Ok(vec![
                    RolePermission {
                        id: 1,
                        user_id: user_id,
                        role: UserRole::Admin,
                    },
                    RolePermission {
                        id: 2,
                        user_id: user_id,
                        role: UserRole::SuperAdmin,
                    }
                ])
            }
        };
    }

    #[tokio::test]
    async fn test_get_user_by_id_pass() {

        static GET_USER_BY_ID: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
        static GET_USER_PERMISSIONS: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, GetUser, get_user)]
        async fn get_user(id: i32) -> Result<User, NanoServiceError> {
            GET_USER_BY_ID.store(true, Ordering::Relaxed);
            assert_eq!(id, 1);
            let new_user = generate_new_user(
                "test@gmail.com".to_string(),
                "test-uuid".to_string(),
            );
            Ok(generate_user(new_user, id))
        }

        impl_roles!(1);

        async fn run_request(req: Request) -> ServiceResponse {
            let service = get_user_by_id::<MockDbHandle, MockConfig, PassAuthSessionCheckMock>;
            let app = init_service(App::new().route("/{id}", web::get().to(service))).await;
            call_service(&app, req).await
        }

        let agent = "some-agent".to_string();

        let jwt: HeaderToken<MockConfig, SuperAdminRoleCheck> = HeaderToken::new(
            agent.clone(), 
            1, 
            UserRole::SuperAdmin,
        );

        let req = TestRequest::get()
            .uri("/1")
            .insert_header(("token", jwt.encode().unwrap()))
            .insert_header((header::USER_AGENT, agent))
            .to_request();

        let resp = run_request(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        let trimmed_user: UserProfile = serde_json::from_str(body_str).unwrap();

        assert_eq!(trimmed_user.user.id, 1);
        assert_eq!(trimmed_user.roles.len(), 2);
        assert_eq!(status, 200);
        assert_eq!(GET_USER_BY_ID.load(Ordering::Relaxed), true);
        assert_eq!(GET_USER_PERMISSIONS.load(Ordering::Relaxed), true);
    }

    #[tokio::test]
    async fn test_get_user_by_email_route() {
        
        static GET_USER_BY_EMAIL: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
        static GET_USER_PERMISSIONS: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, GetUserByEmail, get_user_by_email)]
        async fn get_user_by_email(email: String) -> Result<User, NanoServiceError> {
            GET_USER_BY_EMAIL.store(true, Ordering::Relaxed);
            assert_eq!(email, "test@gmail.com".to_string());
            let new_user = generate_new_user(
                email,
                "test-uuid".to_string(),
            );
            Ok(generate_user(new_user, 2))
        }
        impl_roles!(2);

        async fn run_request(req: Request) -> ServiceResponse {
            let service = get_user_by_email_route::<MockDbHandle, MockConfig, PassAuthSessionCheckMock>;
            let app = init_service(App::new().route("/by-email/{email}", web::get().to(service))).await;
            call_service(&app, req).await
        }

        let agent = "some-agent".to_string();

        let jwt: HeaderToken<MockConfig, SuperAdminRoleCheck> = HeaderToken::new(
            agent.clone(), 
            1, 
            UserRole::SuperAdmin,
        );

        let req = TestRequest::get()
            .uri("/by-email/test@gmail.com")
            .insert_header(("token", jwt.encode().unwrap()))
            .insert_header((header::USER_AGENT, agent))
            .to_request();

        let resp = run_request(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        let trimmed_user: UserProfile = serde_json::from_str(body_str).unwrap();

        assert_eq!(trimmed_user.user.id, 2);
        assert_eq!(trimmed_user.roles.len(), 2);
        assert_eq!(trimmed_user.user.email, "test@gmail.com".to_string());
        assert_eq!(status, 200);
        assert_eq!(GET_USER_BY_EMAIL.load(Ordering::Relaxed), true);
        assert_eq!(GET_USER_PERMISSIONS.load(Ordering::Relaxed), true);

    }

    #[tokio::test]
    async fn test_get_user_by_uuid_route() {
        
        static GET_USER_BY_UUID: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
        static GET_USER_PERMISSIONS: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, GetUserByUuid, get_user_by_uuid)]
        async fn get_user_by_uuid(uuid: String) -> Result<User, NanoServiceError> {
            GET_USER_BY_UUID.store(true, Ordering::Relaxed);
            assert_eq!(uuid, "test-uuid".to_string());
            let new_user = generate_new_user(
                "".to_string(),
                uuid,
            );
            Ok(generate_user(new_user, 3))
        }
        impl_roles!(3);

        async fn run_request(req: Request) -> ServiceResponse {
            let service = get_user_by_uuid_route::<MockDbHandle>;
            let app = init_service(App::new().route("/by-uuid/{uuid}", web::get().to(service))).await;
            call_service(&app, req).await
        }

        let req = TestRequest::get()
            .uri("/by-uuid/test-uuid")
            .to_request();

        let resp = run_request(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        let trimmed_user: UserProfile = serde_json::from_str(body_str).unwrap();

        assert_eq!(trimmed_user.user.id, 3);
        assert_eq!(trimmed_user.user.uuid, "test-uuid".to_string());
        assert_eq!(trimmed_user.roles.len(), 2);
        assert_eq!(status, 200);
        assert_eq!(GET_USER_BY_UUID.load(Ordering::Relaxed), true);
        assert_eq!(GET_USER_PERMISSIONS.load(Ordering::Relaxed), true);
    }

    #[tokio::test]
    async fn test_get_by_jwt() {

        static GET_USER_BY_ID: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
        static GET_USER_PERMISSIONS: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, GetUser, get_user)]
        async fn get_user(id: i32) -> Result<User, NanoServiceError> {
            GET_USER_BY_ID.store(true, Ordering::Relaxed);
            assert_eq!(id, 20);
            let new_user = generate_new_user(
                "".to_string(),
                "test-uuid".to_string(),
            );
            Ok(generate_user(new_user, id))
        }
        impl_roles!(20);

        async fn run_request(req: Request) -> ServiceResponse {
            let service = get_by_jwt::<MockDbHandle, MockConfig, PassAuthSessionCheckMock>;
            let app = init_service(App::new().route("/", web::get().to(service))).await;
            call_service(&app, req).await
        }

        let agent = "some-agent".to_string();

        let jwt: HeaderToken<MockConfig, NoRoleCheck> = HeaderToken::new(
            agent.clone(), 
            20, 
            UserRole::SuperAdmin,
        );

        let req = TestRequest::get()
            .uri("/")
            .insert_header(("token", jwt.encode().unwrap()))
            .insert_header((header::USER_AGENT, agent))
            .to_request();

        let resp = run_request(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        let trimmed_user: UserProfile = serde_json::from_str(body_str).unwrap();

        assert_eq!(trimmed_user.user.id, 20);
        assert_eq!(trimmed_user.user.uuid, "test-uuid".to_string());
        assert_eq!(trimmed_user.roles.len(), 2);
        assert_eq!(status, 200);
        assert_eq!(GET_USER_BY_ID.load(Ordering::Relaxed), true);
        assert_eq!(GET_USER_PERMISSIONS.load(Ordering::Relaxed), true);
    }

}