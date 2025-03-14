//! Gets all the user profiles.
use dal::users::tx_definitions::GetAllUserProfiles;
use kernel::users::UserProfile;
use utils::errors::NanoServiceError;


/// Retrieves all user profiles.
/// 
/// # Returns
/// - `Ok(Vec<UserProfile>)`: If user profiles are found.
pub async fn get_all_user_profiles<X: GetAllUserProfiles>() -> Result<Vec<UserProfile>, NanoServiceError> {
    X::get_all_user_profiles().await
}
