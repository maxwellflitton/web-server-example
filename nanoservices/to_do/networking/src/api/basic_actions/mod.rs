use dal::connections::sqlx_postgres::SqlxPostGresDescriptor;
use utils::config::EnvConfig;
use actix_web::web::{ServiceConfig, scope, post};
mod create;
use kernel::token::session_cache::engine_mem::AuthCacheSessionEngineMem;


pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/todo/v1/basic_actions") // Namespace for user-related API routes.
        .route("create", post().to(
            create::create_to_do_item::<SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>) // POST /api/todo/v1/basic_actions/create.
        )
    );
}
