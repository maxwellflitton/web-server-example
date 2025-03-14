

#[macro_export]
macro_rules! compile_api {
    (|$( $arg_name:ident : $arg_type:ty ),*|{ $($body:tt)* }, $func_name:ident, $($trait_tag:tt)+) => {
        pub async fn $func_name<X>($( $arg_name : $arg_type ),*) -> Result<actix_web::HttpResponse, utils::errors::NanoServiceError> 
        where
            X: $($trait_tag)+
        {
            $($body)*
        }
    };
    (|| { $($body:tt)* }, $func_name:ident, $($trait_tag:tt)+) => {
        pub async fn $func_name<X>() -> Result<actix_web::HttpResponse, utils::errors::NanoServiceError> 
        where
            X: $($trait_tag)+
        {
            $($body)*
        }
    };
    (
        TOKEN($role_check:ty),
        |$( $arg_name:ident : $arg_type:ty ),*| { $($body:tt)* },
        $func_name:ident, 
        $($trait_tag:tt)+) 
        
        => {
        pub async fn $func_name<X, Y, Z>(
            jwt: kernel::token::token::HeaderToken<Y, $role_check>, $( $arg_name : $arg_type ),*
        ) -> Result<actix_web::HttpResponse, utils::errors::NanoServiceError> 
        where
            X: $($trait_tag)+,
            Y: utils::config::GetConfigVariable + Send,
            Z: kernel::token::session_cache::traits::GetAuthCacheSession
        {
            let user_session = match Z::get_auth_cache_session(&jwt).await {
                Ok(Some(session)) => {session},
                Ok(None) => {
                    return Err(utils::errors::NanoServiceError::new(
                        "No longer in session cache".to_string(), 
                        utils::errors::NanoServiceErrorStatus::Unauthorized
                    ))
                },
                Err(e) => {
                    return Err(e)
                }
            };
            $($body)*
        }
    };
    (
        TOKEN($role_check:ty),
        || { $($body:tt)* }, 
        $func_name:ident, 
        $($trait_tag:tt)+) 
        
        => {
        pub async fn $func_name<X, Y, Z>(
            jwt: kernel::token::token::HeaderToken<Y, $role_check>) -> Result<actix_web::HttpResponse, utils::errors::NanoServiceError> 
        where
            X: $($trait_tag)+,
            Y: utils::config::GetConfigVariable + Send,
            Z: kernel::token::session_cache::traits::GetAuthCacheSession
        {
            match Z::get_auth_cache_session(&jwt).await {
                Ok(Some(_)) => {},
                Ok(None) => {
                    return Err(utils::errors::NanoServiceError::new(
                        "No longer in session cache".to_string(), 
                        utils::errors::NanoServiceErrorStatus::Unauthorized
                    ))
                },
                Err(e) => {
                    return Err(e)
                }
            };
            $($body)*
        }
    };
}
