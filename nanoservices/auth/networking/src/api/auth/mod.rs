pub mod login;
pub mod logout;
pub mod request_password_reset;
pub mod refresh;
pub mod resend_confirmation_email;

use dal::connections::sqlx_postgres::SqlxPostGresDescriptor;
use utils::config::EnvConfig;
use email_core::mailchimp_traits::mc_definitions::MailchimpDescriptor;
use actix_web::web::{ServiceConfig, scope, post};
use kernel::token::session_cache::engine_mem::AuthCacheSessionEngineMem;


pub fn auth_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/auth/v1/auth") // Namespace for user-related API routes.
        .route("login", post().to(
            login::login::<SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>) // POST /api/auth/v1/users/login.
        )
        .route("refresh", post().to(
            refresh::refresh::<SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>) // POST /api/auth/v1/users/refresh.
        )
        .route("logout", post().to(
            logout::logout::<AuthCacheSessionEngineMem, EnvConfig>) // POST /api/auth/v1/users/logout.
        )
        .route("request_password_reset", post().to(
            request_password_reset::request_password_reset::<MailchimpDescriptor, SqlxPostGresDescriptor, EnvConfig>) // POST /api/auth/v1/users/password_reset_request.
        )
        .route("resend_confirmation_email", post().to(
            resend_confirmation_email::resend_confirmation_email::<MailchimpDescriptor, SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>) // POST /api/auth/v1/users/resend_confirmation_email.
        )
    );
}
