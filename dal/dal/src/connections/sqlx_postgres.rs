//! Defines the connection to the PostgreSQL database and the `SqlxPostGresDescriptor` for dependency injection.
//!
//! # Overview
//! - Establishes a connection pool for a PostgreSQL database using the `sqlx` library.
//! - Provides the `SqlxPostGresDescriptor` struct to serve as a handle for database-related operations.
//! - Configures the connection pool using environment variables for flexibility and scalability.
//!
//! # Features
//! - The `SQLX_POSTGRES_POOL` is a lazily-initialized static instance for managing database connections.
//! - The `SqlxPostGresDescriptor` is used for dependency injection and applying database traits for transaction handling.
use sqlx::postgres::{PgPool, PgPoolOptions};
use once_cell::sync::Lazy;
use std::env;

/// A descriptor struct used for applying database traits and dependency injection.
///
/// # Notes
/// This struct is intended to be used as a handle for implementing database-related traits
/// that define transactions or other interactions with the database.
pub struct SqlxPostGresDescriptor;

/// A lazily-initialized static instance of the PostgreSQL connection pool.
///
/// # Details
/// - Uses the `DB_URL` environment variable to determine the connection string.
/// - Allows configuring the maximum number of connections via the `TO_DO_MAX_CONNECTIONS` environment variable.
/// - Falls back to a default of 5 maximum connections if the environment variable is not set.
///
/// # Panics
/// - If the `DB_URL` environment variable is not set or the connection pool cannot be created.
pub static SQLX_POSTGRES_POOL: Lazy<PgPool> = Lazy::new(|| {
    // Retrieve the database connection string from the environment.
    let connection_string = env::var("DB_URL").unwrap();

    // Determine the maximum number of connections from the environment.
    let max_connections = match std::env::var("TO_DO_MAX_CONNECTIONS") {
        Ok(val) => val,
        Err(_) => "5".to_string(), // Default to 5 if not set.
    }
    .trim()
    .parse::<u32>()
    .map_err(|_e| "Could not parse max connections".to_string())
    .unwrap();

    // Configure the connection pool.
    let pool = PgPoolOptions::new()
        .max_connections(max_connections);

    // Establish the connection pool lazily.
    pool.connect_lazy(&connection_string)
        .expect("Failed to create pool")
});
