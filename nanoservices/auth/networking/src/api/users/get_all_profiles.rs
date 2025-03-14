//! Endpoint that gets all the user profiles.
use actix_web::HttpResponse;
use auth_core::api::users::get_all_profiles::get_all_user_profiles as get_all_user_profiles_core;
use dal::users::tx_definitions::GetAllUserProfiles;
use utils::api_endpoint;


#[api_endpoint(token=SuperAdminRoleCheck, db_traits=[GetAllUserProfiles])]
pub async fn get_all_user_profiles() {
    let user_profiles = get_all_user_profiles_core::<X>().await?;
    Ok(HttpResponse::Ok().json(user_profiles))
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
    use kernel::users::UserRole;
    use kernel::role_permissions::RolePermission;
    use kernel::users::{UserProfile, TrimmedUser};
    use utils::errors::NanoServiceError;
    use kernel::token::token::HeaderToken;
    use kernel::token::session_cache::engine_mock::PassAuthSessionCheckMock;
    use utils::config::GetConfigVariable;
    use kernel::token::checks::SuperAdminRoleCheck;


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

    #[tokio::test]
    async fn test_get_all_user_profiles_success() {
        struct MockDbHandle;

        #[impl_transaction(MockDbHandle, GetAllUserProfiles, get_all_user_profiles)]
        async fn get_all_user_profiles() -> Result<Vec<UserProfile>, NanoServiceError> {
            Ok(vec![
                UserProfile {
                    user: TrimmedUser::from(generate_user(
                        generate_new_user("test@gmail.com".to_string(), "1".to_string()), 
                        1
                    )),
                    role_permissions: vec![
                        RolePermission {
                            id: 1,
                            role: UserRole::Admin,
                            user_id: 1,
                        },
                        RolePermission {
                            id: 2,
                            role: UserRole::SuperAdmin,
                            user_id: 1,
                        }
                    ]
                },
                UserProfile {
                    user: TrimmedUser::from(generate_user(
                        generate_new_user("testing@gmail.com".to_string(), "2".to_string()), 
                        2
                    )),
                    role_permissions: vec![
                        RolePermission {
                            id: 1,
                            role: UserRole::Admin,
                            user_id: 2,
                        },
                        RolePermission {
                            id: 2,
                            role: UserRole::SuperAdmin,
                            user_id: 2,
                        }
                    ]
                }
            ])
        }

        async fn run_request(req: Request) -> ServiceResponse {
            let service = get_all_user_profiles::<MockDbHandle, MockConfig, PassAuthSessionCheckMock>;
            let app = init_service(App::new().route("/get", web::get().to(service))).await;
            call_service(&app, req).await
        }

        let agent = "some-agent".to_string();

        let jwt: HeaderToken<MockConfig, SuperAdminRoleCheck> = HeaderToken::new(
            agent.clone(), 
            1, 
            UserRole::SuperAdmin,
        );

        let req = TestRequest::get()
            .uri("/get")
            .insert_header(("token", jwt.encode().unwrap()))
            .insert_header((header::USER_AGENT, agent))
            .to_request();

        let resp = run_request(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        let user_profiles: Vec<UserProfile> = serde_json::from_str(body_str).unwrap();
        assert_eq!(status, 200);
        assert_eq!(user_profiles.len(), 2);
    }

}