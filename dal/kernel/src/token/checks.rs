//! This module defines the checks that the token service uses to validate user roles.
//! 
//! # Notes
//! The `$match_expr:pat` is used as opposed to `$match_expr:expr` to allow for the use of the `|` operator.
//! The `$(,)?` is used to allow for the optional trailing comma in the macro.
use crate::users::UserRole;
use utils::errors::{NanoServiceError, NanoServiceErrorStatus};


macro_rules! construct_checks {
    ($( $struct:ident => $match_expr:pat),* $(,)?) => {
        $(
            pub struct $struct;

            impl CheckUserRole for $struct {
                fn check_user_role(role: &UserRole) -> Result<(), NanoServiceError> {
                    match role {
                        $match_expr => Ok(()),
                        _ => Err(NanoServiceError {
                            status: NanoServiceErrorStatus::Unauthorized,
                            message: "Role does not have sufficient permissions".to_string()
                        })
                    }
                }
            }
        )*
    };
}


pub trait CheckUserRole {
    fn check_user_role(role: &UserRole) -> Result<(), NanoServiceError>;
}

construct_checks!(
    SuperAdminRoleCheck => UserRole::SuperAdmin,
    AdminRoleCheck => UserRole::SuperAdmin | UserRole::Admin,
    WorkerRoleCheck => UserRole::SuperAdmin | UserRole::Admin | UserRole::Worker,
    NoRoleCheck => UserRole::SuperAdmin | UserRole::Admin | UserRole::Worker,
    ExactSuperAdminRoleCheck => UserRole::SuperAdmin,
    ExactAdminRoleCheck => UserRole::Admin,
    ExactWorkerRoleCheck => UserRole::Worker
);
