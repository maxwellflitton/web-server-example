use utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use base64::{Engine as _, engine::general_purpose};
use actix_web::HttpRequest;


/// Extracts the basic auth credentials from the request.
/// 
/// # Arguments
/// req: the request to extract the credentials from
/// 
/// # Returns
/// A tuple containing the email and password
pub fn extract_basic_auth_credentials(req: &HttpRequest) -> Result<(String, String), NanoServiceError> {
    // Extract the Authorization header
    let auth_header = match req.headers().get("Authorization"){
        Some(header) => header,
        None => return Err(
            NanoServiceError::new("No Authorization header found".to_string(), NanoServiceErrorStatus::Unauthorized)
        )
    };
    // Convert the header value to a string
    let auth_str = match auth_header.to_str().ok() {
        Some(header) => header,
        None => return Err(
            NanoServiceError::new("Invalid Authorization header".to_string(), NanoServiceErrorStatus::Unauthorized)
        )
    };
    // Check if the header starts with "Basic "
    if !auth_str.starts_with("Basic ") {
        return Err(
            NanoServiceError::new("Invalid Authorization header".to_string(), NanoServiceErrorStatus::Unauthorized)
        )
    }
    // Extract the encoded credentials part and decode from Base64
    let encoded_credentials = &auth_str["Basic ".len()..];
    let decoded_bytes = match general_purpose::STANDARD.decode(encoded_credentials) {
        Ok(decoded) => decoded,
        Err(e) => return Err(
            NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unauthorized)
        )
    };
    let decoded_str = match String::from_utf8(decoded_bytes) {
        Ok(decoded) => decoded,
        Err(_) => return Err(
            NanoServiceError::new("Invalid Authorization header".to_string(), NanoServiceErrorStatus::Unauthorized)
        )
    };
    // Split the decoded string into username and password
    let mut credentials = decoded_str.splitn(2, ':');
    let username = match credentials.next(){
        Some(username) => username,
        None => return Err(
            NanoServiceError::new("Invalid Authorization header".to_string(), NanoServiceErrorStatus::Unauthorized)
        )
    };
    let password = match credentials.next() {
        Some(password) => password,
        None => return Err(
            NanoServiceError::new("Invalid Authorization header".to_string(), NanoServiceErrorStatus::Unauthorized)
        )
    };
    Ok((username.to_string(), password.to_string()))
}