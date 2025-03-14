//! Defines transaction traits for interacting with the database.
//!
//! # Overview
//! This file uses a macro to define traits for database transactions. These traits specify the 
//! actions that can be performed on the `User` entities, such as creating, retrieving, deleting, 
//! and confirming users. Each transaction corresponds to a specific database operation and is 
//! designed to be implemented by the data access layer (DAL).
//!
//! # Purpose
//! - Streamlines the creation of traits for database transactions.
//! - Provides a consistent interface for interacting with `User` entities in the database.
//! - Supports dependency injection and ensures flexibility when passing these traits to core 
//!   functions or services.
use crate::define_dal_transactions;
use kernel::users::{NewUser, User, UserProfile};


define_dal_transactions!(
    CreateUser => create_user(user: NewUser) -> User,
    GetUser => get_user(id: i32) -> User,
    GetUserByEmail => get_user_by_email(email: String) -> User,
    GetUserByUuid => get_user_by_uuid(uuid: String) -> User,
    DeleteUser => delete_user(id: i32) -> bool,
    ConfirmUser => confirm_user(uuid: String) -> bool,
    GetUserProfileByEmail => get_user_profile_by_email(email: String) -> UserProfile,
    GetAllUserProfiles => get_all_user_profiles() -> Vec<UserProfile>,
    BlockUser => block_user(id: i32) -> bool,
    UnblockUser => unblock_user(id: i32) -> bool,
    ResetPassword => reset_password(uuid: String, new_password: String) -> bool,
    UpdateUuid => update_uuid(email: String, new_uuid: String) -> bool,
    UpdateUserUsername => update_user_username(id: i32, username: String) -> bool,
    UpdateUserEmail => update_user_email(id: i32, email: String) -> bool,
    UpdateUserFirstName => update_user_first_name(id: i32, first_name: String) -> bool,
    UpdateUserLasttName => update_user_last_name(id: i32, last_name: String) -> bool,
);
