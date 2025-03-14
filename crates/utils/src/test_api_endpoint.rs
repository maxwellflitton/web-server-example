//! Defines the macro for sending a test request. Below is an example:
//! 
//! send_test_request!(
//!     POST,                       // method 
//!     "/assign_role",             // endpoint
//!     serde_json::json!({         // body
//!         "user_id": 2,
//!         "role": "Admin"
//!     }),
//!     SuperAdminRoleCheck,        // check for the JWT that is sent
//!     UserRole::SuperAdmin,      // role for the JWT
//!     1,                         // user id associated with the JWT
//!     MockPostgres, MockConfig, PassAuthSessionCheckMock // traits slotted into the API endpoint function being tested
//!);
//!
//!let resp = send_request().await; // the send_request function is returned and we just call that to get the response
//! 

#[macro_export]
macro_rules! send_test_request {
    (POST, $endpoint:expr, $body:expr, $check:ident, $role:path, $id:expr, $func_name:ident, $($trait_tag:tt)+) => {
        struct MockConfig;

        impl utils::config::GetConfigVariable for MockConfig {
            fn get_config_variable(_key: String) -> Result<String, utils::errors::NanoServiceError> {
                Ok("secret".to_string())
            }
        }

        async fn run_request(req: actix_http::Request) -> actix_web::dev::ServiceResponse {
            // let service = assign_role::<MockPostgres, MockConfig, kernel::token::session_cache::engine_mock::PassAuthSessionCheckMock>;
            let service = $func_name::<$($trait_tag)+>;
            let app = actix_web::test::init_service(actix_web::App::new().route($endpoint, actix_web::web::post().to(service))).await;
            actix_web::test::call_service(&app, req).await
        }
        async fn send_request() -> actix_web::dev::ServiceResponse {
            let body = $body;
            let agent = "some-agent".to_string();
            let jwt: kernel::token::token::HeaderToken<MockConfig, $check> = kernel::token::token::HeaderToken::new(
                agent.clone(), 
                $id, 
                $role,
            );
            let req = actix_web::test::TestRequest::post()
                .insert_header(actix_web::http::header::ContentType::json())
                .insert_header(("token", jwt.encode().unwrap()))
                .insert_header((actix_web::http::header::USER_AGENT, agent))
                .uri($endpoint)
                .set_json(&body)
                .to_request();

            run_request(req).await
        }
    };
    (GET, $endpoint:expr, $body:expr, $check:ident, $role:path, $id:expr, $($trait_tag:tt)+) => {
        struct MockConfig;

        impl utils::config::GetConfigVariable for MockConfig {
            fn get_config_variable(_key: String) -> Result<String, utils::errors::NanoServiceError> {
                Ok("secret".to_string())
            }
        }

        async fn run_request(req: actix_http::Request) -> actix_web::dev::ServiceResponse {
            // let service = assign_role::<MockPostgres, MockConfig, kernel::token::session_cache::engine_mock::PassAuthSessionCheckMock>;
            let service = assign_role::<$($trait_tag)+>;
            let app = actix_web::test::init_service(actix_web::App::new().route($endpoint, actix_web::web::get().to(service))).await;
            actix_web::test::call_service(&app, req).await
        }

        async fn send_request() -> actix_web::dev::ServiceResponse {
            let body = $body;
            let agent = "some-agent".to_string();
            let jwt: kernel::token::token::HeaderToken<MockConfig, $check> = kernel::token::token::HeaderToken::new(
                agent.clone(), 
                $id, 
                $role,
            );
            let req = actix_web::test::TestRequest::post()
                .insert_header(actix_web::http::header::ContentType::json())
                .insert_header(("token", jwt.encode().unwrap()))
                .insert_header((actix_web::http::header::USER_AGENT, agent))
                .uri($endpoint)
                .set_json(&body)
                .to_request();

            run_request(req).await
        }
    };
}