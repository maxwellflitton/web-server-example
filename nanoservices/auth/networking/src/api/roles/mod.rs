pub mod assign_role;
pub mod remove_role;
pub mod update_roles;

use dal::connections::sqlx_postgres::SqlxPostGresDescriptor;
use utils::config::EnvConfig;
use actix_web::web::{ServiceConfig, scope, post};
use kernel::token::session_cache::engine_mem::AuthCacheSessionEngineMem;


pub fn roles_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/auth/v1/roles") // Namespace for user-related API routes.
        .route("assign_role", post().to(
            assign_role::assign_role::<SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>) // POST /api/auth/v1/roles/assign_role.
        )
        .route("remove_role", post().to(
            remove_role::remove_role::<SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>) // POST /api/auth/v1/roles/remove_role.
        )
        .route("update", post().to(
            update_roles::update_roles::<SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>) // POST /api/auth/v1/roles/update.
        )
    );
}