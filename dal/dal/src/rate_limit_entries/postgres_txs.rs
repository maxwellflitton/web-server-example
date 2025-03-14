//! Implements transaction traits for PostgreSQL using the `SqlxPostGresDescriptor`.
//!
//! # Overview
//! This file implements the email rate limit related transaction traits (`CreateRateLimitEntry`, 
//! `GetRateLimitEntry`, `UpdateRateLimitEntry`) for PostgreSQL using the `SqlxPostGresDescriptor`.
//! Each implementation maps the transaction to a specific database operation.
use dal_tx_impl::impl_transaction;
use kernel::rate_limit_entries::{RateLimitEntry, NewRateLimitEntry};
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use crate::connections::sqlx_postgres::{SQLX_POSTGRES_POOL, SqlxPostGresDescriptor};
use crate::rate_limit_entries::tx_definitions::{CreateRateLimitEntry, GetRateLimitEntry, UpdateRateLimitEntry};

/// Implements the `CreateRateLimitEntry` trait for the `SqlxPostGresDescriptor`.
///
/// Inserts a new rate limit entry into the PostgreSQL database and returns the created rate limit entry
#[impl_transaction(SqlxPostGresDescriptor, CreateRateLimitEntry, create_rate_limit_entry)]
async fn create_rate_limit_entry(email: NewRateLimitEntry) -> Result<RateLimitEntry, NanoServiceError> {
    let query = r#"
        INSERT INTO rate_limit_entries (email, count)
        VALUES ($1, $2)
        RETURNING id, email, rate_limit_period_start, count
    "#;

    sqlx::query_as::<_, RateLimitEntry>(query)
        .bind(email.email)
        .bind(1)
        .fetch_one(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(format!("Failed to create rate limit entry: {}", e), NanoServiceErrorStatus::Unknown))
}

/// Implements the `GetRateLimitEntry` trait for the `SqlxPostGresDescriptor`.
///
/// Gets a rate limit entry from the PostgreSQL database.
#[impl_transaction(SqlxPostGresDescriptor, GetRateLimitEntry, get_rate_limit_entry)]
async fn get_rate_limit_entry(email: String) -> Result<Option<RateLimitEntry>, NanoServiceError> {
    let query = r#"
        SELECT id, email, rate_limit_period_start, count 
        FROM rate_limit_entries
        WHERE email = $1
    "#;

    let result = sqlx::query_as::<_, RateLimitEntry>(query)
        .bind(email)
        .fetch_optional(&*SQLX_POSTGRES_POOL) // Use fetch_optional here
        .await
        .map_err(|e| NanoServiceError::new(
            format!("Failed to fetch rate limit entry: {}", e),
            NanoServiceErrorStatus::Unknown,
        ))?;

    Ok(result)
}

/// Implements the `UpdateRateLimitEntry` trait for the `SqlxPostGresDescriptor`.
///
/// Updates all fields of a rate limit entry for the given ID.
#[impl_transaction(SqlxPostGresDescriptor, UpdateRateLimitEntry, update_rate_limit_entry)]
async fn update_rate_limit_entry(updated_entry: RateLimitEntry) -> Result<bool, NanoServiceError> {
    let query = r#"
        UPDATE users
        SET rate_limit_period_start = $1, count = $2
        WHERE id = $3
    "#;

    let result = sqlx::query(query)
        .bind(updated_entry.rate_limit_period_start)
        .bind(updated_entry.count)
        .bind(updated_entry.id)
        .execute(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| {
            NanoServiceError::new(
                format!("Failed to update rate limit entry: {}", e),
                NanoServiceErrorStatus::Unknown,
            )
        })?;

    Ok(result.rows_affected() > 0)
}

