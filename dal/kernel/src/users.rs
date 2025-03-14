//! Defines the `NewUser`, `User`, and `TrimmedUser` structs for managing users in the system.
//!
//! This file provides data structures and utility methods for user creation, password hashing,
//! verification, and data interaction between the kernel workspace and the data access layer.
//!
//! ## Purpose
//! - Enable database interactions through `User` and `TrimmedUser` structs.
//! - Provide utility functions for password management.
//! - Support service-level operations with these structs.
//!
//! ## Notes
//! - Passwords are hashed using the Argon2 algorithm.
//! - The `NanoServiceError` is used for consistent error handling.
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use argon2::{
    Argon2, 
    PasswordHasher, 
    PasswordVerifier, 
    password_hash::{
        SaltString, 
        PasswordHash,
    }
};
use sqlx::postgres::PgTypeInfo;
use sqlx::{Decode, Encode, Postgres, Type};
use std::str::FromStr;
use std::error::Error;
use crate::role_permissions::RolePermission;
use rand::Rng;


/// Utility function for creating a hashed password.
///
/// # Arguments
/// * `password` - The password to hash.
/// 
/// # Returns
/// * `Ok(String)` - The hashed password if successful.
pub fn hash_password(password: String) -> Result<String, NanoServiceError> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2_hasher = Argon2::default();
    let hashed_password =  argon2_hasher.hash_password(
        password.as_bytes(),
        &salt,
    ).map_err(|e| {
        NanoServiceError::new(
            format!("Failed to hash password: {}", e),
            NanoServiceErrorStatus::Unknown,
        )
    })?.to_string();
    Ok(hashed_password)
}


/// Represents the schema for creating a new user in the system.
/// 
/// # Notes
/// 
/// This is for the incoming request body when creating a new user.
/// 
/// # Fields
/// * `email` - The email address of the new user.
/// * `password` - The plaintext password of the new user.
/// * `first_name` - The first name of the new user.
/// * `last_name` - The last name of the new user.
/// * `user_role` - The role assigned to the user.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUserSchema {
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub user_role: UserRole
}

impl NewUserSchema {
    /// Converts a `NewUserSchema` into a `NewUser`.
    /// 
    /// # Returns
    /// * `Ok(NewUser)` - If the conversion is successful.
    pub fn to_new_user(self) -> Result<NewUser, NanoServiceError> {
        let rng = rand::thread_rng();  // Create a random number generator
        let random_string: String = rng
            .sample_iter(&rand::distributions::Alphanumeric)  // Use the Alphanumeric distribution
            .take(10)                        // Take 'length' number of random characters
            .map(char::from)               // Convert each byte to a char
            .collect();                                               // Collect the chars into a String
        NewUser::new(
            self.username,
            self.email,
            self.first_name,
            self.last_name,
            self.user_role,
            random_string
        )
    }
}


/// Represents the role assigned to a user in the system.
/// 
/// # Variants
/// * `SuperAdmin` - The super administrator role who has full control over the system.
/// * `Admin` - The administrator role who can oversee and perform actions on workers such as block, invite, delete.
///             They will also be able to assign tasks to workers and inspect progress.
/// * `Worker` - The worker role who can perform tasks assigned by the administrator.
#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    SuperAdmin,
    Admin,
    Worker,
    Unreachable
}

// Manually implement `sqlx::Type` to match TEXT type
impl Type<Postgres> for UserRole {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("VARCHAR")
    }
}

// Implement `sqlx::Encode` for inserting into Postgres
impl Encode<'_, Postgres> for UserRole {
    fn encode_by_ref(&self, buf: &mut <Postgres as sqlx::Database>::ArgumentBuffer<'_>) -> Result<sqlx::encode::IsNull, Box<dyn Error + Sync + Send>> {
        let role_str = match self {
            UserRole::SuperAdmin => "Super Admin",
            UserRole::Admin => "Admin",
            UserRole::Worker => "Worker",
            UserRole::Unreachable => "Unreachable",
        };
        <&str as Encode<Postgres>>::encode(role_str, buf)
    }
}

// Implement `sqlx::Decode` for retrieving from Postgres
impl<'r> Decode<'r, Postgres> for UserRole {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        UserRole::from_str(s).map_err(|e| e.into())
    }
}

// Implement `FromStr` for easy conversion
impl FromStr for UserRole {
    type Err = String;
    fn from_str(role: &str) -> Result<Self, Self::Err> {
        match role.trim() {
            "Super Admin" => Ok(UserRole::SuperAdmin),
            "Admin" => Ok(UserRole::Admin),
            "Worker" => Ok(UserRole::Worker),
            "Unreachable" => Ok(UserRole::Unreachable),
            _ => Err(format!("Invalid user role: {}", role)),
        }
    }
}


impl Serialize for UserRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let role = match self {
            UserRole::Admin => "Admin",
            UserRole::Worker => "Worker",
            UserRole::SuperAdmin => "Super Admin",
            UserRole::Unreachable => "Unreachable"
        };
        serializer.serialize_str(role)
    }
}

impl<'de> serde::Deserialize<'de> for UserRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let role: String = serde::Deserialize::deserialize(deserializer)?;
        UserRole::from_string(&role).map_err(serde::de::Error::custom)
    }
}



impl UserRole {

    /// Converts the `UserRole` into a string.
    /// 
    /// # Returns
    /// * `String` - The string representation of the role.
    pub fn to_string(&self) -> String {
        match self {
            UserRole::Admin => "Admin".to_string(),
            UserRole::Worker => "Worker".to_string(),
            UserRole::SuperAdmin => "Super Admin".to_string(),
            UserRole::Unreachable => "Unreachable".to_string()
        }
    }

    /// Converts a string into a `UserRole`.
    /// 
    /// # Arguments
    /// * `role` - The string representation of the role.
    /// 
    /// # Returns
    /// * `Ok(UserRole)` - If the conversion is successful.
    pub fn from_string(role: &str) -> Result<UserRole, NanoServiceError> {
        match role.to_lowercase().as_str() {
            "admin" => Ok(UserRole::Admin),
            "worker" => Ok(UserRole::Worker),
            "super admin" => Ok(UserRole::SuperAdmin),
            _ => Err(NanoServiceError::new(
                format!("Invalid user role: {}", role),
                NanoServiceErrorStatus::BadRequest,
            )),
        }
    }
}


/// Represents a new user being created in the system.
///
/// # Fields
/// * `username` - The username of the user.
/// * `email` - The email address of the user.
/// * `password` - The hashed password of the user.
/// * `first_name` - The first name of the user.
/// * `last_name` - The last name of the user.
/// * `user_role` - The role assigned to the user.
/// * `date_created` - The date and time the user was created.
/// * `last_logged_in` - The date and time the user last logged in.
/// * `blocked` - A boolean indicating if the user is blocked.
/// * `uuid` - A unique identifier for the user.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub confirmed: bool,
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub user_role: UserRole,
    pub date_created: NaiveDateTime,
    pub last_logged_in: NaiveDateTime,
    pub blocked: bool,
    pub uuid: String,
}

impl NewUser {

    /// Creates a new user by hashing the password and generating a unique identifier.
    ///
    /// # Arguments
    /// * `email` - The email address of the new user.
    /// * `password` - The plaintext password of the new user.
    /// * `first_name` - The first name of the user.
    /// * `last_name` - The last name of the user.
    /// * `user_role` - The role assigned to the user.
    ///
    /// # Returns
    /// * `Ok(NewUser)` - If the user is successfully created.
    /// * `Err(NanoServiceError)` - If password hashing fails.
    ///
    /// # Notes
    /// - Uses Argon2 for password hashing.
    pub fn new(
        username: String,
        email: String,
        first_name: String,
        last_name: String,
        user_role: UserRole,
        password: String,
    ) -> Result<NewUser, NanoServiceError> {

        let hash = hash_password(password)?;
        let now = chrono::Utc::now().naive_utc();

        Ok(NewUser {
            confirmed: false,
            username,
            email,
            password: hash,
            first_name,
            last_name,
            user_role,
            date_created: now,
            last_logged_in: now,
            blocked: false,
            uuid: uuid::Uuid::new_v4().to_string(),
        })
    }
}


/// Represents a user record retrieved from the database.
///
/// # Fields
/// * `id` - The unique identifier of the user in the database.
/// * `confirmed` - A boolean indicating if the user's email is confirmed.
/// * `username` - The username of the user.
/// * `email` - The email address of the user.
/// * `password` - The hashed password of the user.
/// * `first_name` - The first name of the user.
/// * `last_name` - The last name of the user.
/// * `user_role` - The role assigned to the user.
/// * `date_created` - The date and time the user was created.
/// * `last_logged_in` - The date and time the user last logged in.
/// * `blocked` - A boolean indicating if the user is blocked.
/// * `uuid` - A unique identifier for the user.
#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub confirmed: bool,
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub user_role: UserRole,
    pub date_created: NaiveDateTime,
    pub last_logged_in: NaiveDateTime,
    pub blocked: bool,
    pub uuid: String,
}

impl User {
    /// Verifies if a provided plaintext password matches the stored hashed password.
    ///
    /// # Arguments
    /// * `password` - The plaintext password to verify.
    ///
    /// # Returns
    /// * `Ok(true)` - If the password is valid.
    /// * `Ok(false)` - If the password is invalid.
    /// * `Err(NanoServiceError)` - If the hash parsing fails.
    ///
    /// # Notes
    /// - Uses Argon2 for password verification.
    pub fn verify_password(&self, password: String) -> Result<bool, NanoServiceError> {
        let argon2_hasher = Argon2::default();
        let parsed_hash = PasswordHash::new(&self.password).map_err(|e| {
            NanoServiceError::new(
                format!("Failed to parse password hash: {}", e),
                NanoServiceErrorStatus::Unknown,
            )
        })?;
        let is_valid = argon2_hasher.verify_password(password.as_bytes(), &parsed_hash).is_ok();
        Ok(is_valid)
    }
}

/// Represents a lightweight version of a user for data transfer purposes.
///
/// # Fields
/// * `id` - The unique identifier of the user in the database.
/// * `confirmed` - A boolean indicating if the user's email is confirmed.
/// * `username` - The username of the user.
/// * `email` - The email address of the user.
/// * `first_name` - The first name of the user.
/// * `last_name` - The last name of the user.
/// * `user_role` - The role assigned to the user.
/// * `date_created` - The date and time the user was created.
/// * `last_logged_in` - The date and time the user last logged in.
/// * `blocked` - A boolean indicating if the user is blocked.
/// * `uuid` - A unique identifier for the user.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TrimmedUser {
    pub id: i32,
    pub confirmed: bool,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub user_role: UserRole,
    pub date_created: NaiveDateTime,
    pub last_logged_in: NaiveDateTime,
    pub blocked: bool,
    pub uuid: String,
}

impl From<User> for TrimmedUser {
    /// Converts a `User` into a `TrimmedUser`.
    ///
    /// # Arguments
    /// * `user` - The full user struct to be trimmed.
    ///
    /// # Returns
    /// * `TrimmedUser` - A lightweight version of the user.
    fn from(user: User) -> Self {
        TrimmedUser {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            user_role: user.user_role,
            date_created: user.date_created,
            last_logged_in: user.last_logged_in,
            blocked: user.blocked,
            uuid: user.uuid,
            confirmed: user.confirmed,
        }
    }
}


/// Represents a user profile with role permissions.
/// 
/// # Fields
/// * `user` - The trimmed user details.
/// * `role_permissions` - The permissions assigned to the user's role.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UserProfile {
    pub user: TrimmedUser,
    pub role_permissions: Vec<RolePermission>,
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hash_password_success() {
        let password = "pasword".to_string();
        let result = hash_password(password.clone());

        assert!(result.is_ok(), "Expected Ok, but got an error");
        let hashed = result.unwrap();
        assert_ne!(hashed, password, "Hashed password should not match the original");
    }
    
    #[test]
    fn test_user_role_serialization() {
        // Test serialization
        let admin_role = UserRole::Admin;
        let worker_role = UserRole::Worker;

        let admin_json = serde_json::to_string(&admin_role).expect("Failed to serialize Admin role");
        let worker_json = serde_json::to_string(&worker_role).expect("Failed to serialize Worker role");

        assert_eq!(admin_json, "\"Admin\"");
        assert_eq!(worker_json, "\"Worker\"");

        // Test deserialization
        let admin_deserialized: UserRole =
            serde_json::from_str(&admin_json).expect("Failed to deserialize Admin role");
        let worker_deserialized: UserRole =
            serde_json::from_str(&worker_json).expect("Failed to deserialize Worker role");

        assert_eq!(admin_deserialized, UserRole::Admin);
        assert_eq!(worker_deserialized, UserRole::Worker);

        // test all highercase

        let admin_role = "\"ADMIN\"";
        let worker_role = "\"wORker\"";

        let admin_deserialized: UserRole = serde_json::from_str(admin_role).expect("Failed to deserialize Admin role");
        let worker_deserialized: UserRole = serde_json::from_str(worker_role).expect("Failed to deserialize Worker role");
        assert_eq!(admin_deserialized, UserRole::Admin);
        assert_eq!(worker_deserialized, UserRole::Worker);
    }

    #[test]
    fn test_verify_password() {
        use chrono::Utc;
        
        // Input data
        let username = "testuser".to_string();
        let email = "test@example.com".to_string();
        let plaintext_password = "securepassword".to_string();
        let first_name = "John".to_string();
        let last_name = "Doe".to_string();
        let user_role = UserRole::Worker;

        // Create a new user using the `NewUser::new` method
        let new_user = NewUser::new(
            username.clone(),
            email.clone(),
            first_name.clone(),
            last_name.clone(),
            user_role.clone(),
            plaintext_password.clone(),
        )
        .expect("Failed to create NewUser");

        // Ensure the password is hashed and is not the plaintext password
        assert_ne!(new_user.password, plaintext_password);

        // Simulate retrieving the user from the database
        let user = User {
            id: 1,
            confirmed: false,
            username: new_user.username.clone(),
            email: new_user.email.clone(),
            password: new_user.password.clone(),
            first_name: new_user.first_name.clone(),
            last_name: new_user.last_name.clone(),
            user_role: new_user.user_role.clone(),
            date_created: Utc::now().naive_utc(),
            last_logged_in: Utc::now().naive_utc(),
            blocked: new_user.blocked,
            uuid: new_user.uuid.clone(),
        };

        // Verify the password using the `User::verify_password` method
        let is_valid = user
            .verify_password(plaintext_password)
            .expect("Password verification failed");

        // Assert that the password is valid
        assert!(is_valid, "Password verification failed");

        // Test invalid password
        let is_invalid = user
            .verify_password("invalidpassword".to_string())
            .expect("Password verification failed");

        // Assert that the password is invalid
        assert_eq!(is_invalid, false, "Password verification failed");
    }

}