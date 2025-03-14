//! Defines transaction traits for interacting with the `RateLimitEntry` database table.
//!
//! # Overview
//! This file uses the `define_dal_transactions` macro to create traits for database transactions
//! specific to the `RateLimitEntry` entities. Each trait represents a distinct database operation such as
//! creating, updating, and getting email rate limit entries.
//!
//! ## Purpose
//! - Provide an interface for core logic to interact with the data access layer (DAL).
//! - Support dependency injection for database transaction implementations.
//!
//! ## Notes
//! - These traits are designed to be implemented by database descriptor structs, such as `SqlxPostGresDescriptor`.
use kernel::rate_limit_entries::{RateLimitEntry, NewRateLimitEntry};
use crate::define_dal_transactions;


define_dal_transactions!(
    CreateRateLimitEntry => create_rate_limit_entry(new_entry: NewRateLimitEntry) -> RateLimitEntry,
    GetRateLimitEntry => get_rate_limit_entry(email: String) -> Option<RateLimitEntry>,
    UpdateRateLimitEntry => update_rate_limit_entry(updated_entry: RateLimitEntry) -> bool,
);
