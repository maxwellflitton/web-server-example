//! Defines API endpoints for user-related operations.
//!
//! # Overview
//! This module sets up and configures the API routes for user-related actions under the `/api/auth/v1/users` namespace.
//! It delegates functionality to individual modules for specific endpoints, such as `create`.
//!
//! # Features
//! - Uses Actix Web for defining and mounting HTTP routes.
//! - Integrates with the data access layer (DAL) using the `SqlxPostGresDescriptor`.
//! - Follows a modular structure where each endpoint has its own module for separation of concerns.

pub mod create;
pub mod create_super_admin;
pub mod block;
pub mod unblock;
pub mod get;
pub mod get_all_profiles;
pub mod confirm_user;
pub mod reset_password;
pub mod update;
pub mod delete;

use dal::connections::sqlx_postgres::SqlxPostGresDescriptor;
use actix_web::web::{ServiceConfig, scope, post, get};
use utils::config::EnvConfig;
use kernel::token::session_cache::engine_mem::AuthCacheSessionEngineMem;
use email_core::mailchimp_traits::mc_definitions::MailchimpDescriptor;

/// Configures the API routes for user-related operations.
///
/// # Arguments
/// - `app`: A mutable reference to the `ServiceConfig` to which the routes are added.
///
/// # Routes
/// - `POST /api/auth/v1/users/create`: Creates a new user using the `create` module.
///
/// # Example
/// ```rust
/// use actix_web::{web, App, HttpServer};
/// use my_package::users_factory;
///
/// HttpServer::new(|| {
///     App::new().configure(users_factory)
/// })
/// .bind("127.0.0.1:8080")?
/// .run()
/// .await?;
/// ```
pub fn users_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/auth/v1/users") // Namespace for user-related API routes.
        .route("create/superadmin", post().to(
            create_super_admin::create_super_user::<MailchimpDescriptor, SqlxPostGresDescriptor, EnvConfig>) // POST /api/auth/v1/users/create.
        )
        .route("update", post().to(
            update::update::<SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>) // POST /api/auth/v1/users/update.
        )
        .route("create", post().to(
            create::create_user::<MailchimpDescriptor, SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>) // POST /api/auth/v1/users/create.
        )
        .route("delete", post().to(
            delete::delete_user::<SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>) // POST /api/auth/v1/users/delete.
        )
        .route("block", post().to(
            block::block_user::<SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>) // POST /api/auth/v1/users/block.
        )
        .route("unblock", post().to(
            unblock::unblock_user::<SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>) // POST /api/auth/v1/users/unblock.
        )
        .route("get-by-id/{id}", get().to(
            get::get_user_by_id::<SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>)
        )
        .route("/get-by-email/{email}", get().to(
            get::get_user_by_email_route::<SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>)
        )
        .route("/get-by-uuid/{uuid}", get().to(
            get::get_user_by_uuid_route::<SqlxPostGresDescriptor>)
        )
        .route("/get-by-jwt", get().to(
            get::get_by_jwt::<SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>)
        )
        .route("/get-all", get().to(
            get_all_profiles::get_all_user_profiles::<SqlxPostGresDescriptor, EnvConfig, AuthCacheSessionEngineMem>)
        )
        .route("/confirm", post().to(
            confirm_user::confirm_user::<SqlxPostGresDescriptor>)
        )
        .route("/reset-password", post().to(
            reset_password::reset_password::<SqlxPostGresDescriptor>)
        )
    );
}
