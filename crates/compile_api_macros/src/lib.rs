// ! This macro handles boilerplate for API endpoints, particularly around tokens and two sets of traits:
// ! `db_traits` for database-related functionality, and `email_traits` for email-related functionality.
// ! By specifying these traits, you can automatically get a type parameter for your function (`W`, `X` etc.)
// ! with the trait bounds you've listed. You can then call or reference functions that require those traits
// ! inside your endpoint.
// ! 
// ! Below are the common usage patterns.
// ! 
// ! ## Endpoint with no token or DAL
// ! With no `db_traits` (DAL) and no token, you can write:
// ! ```no_run
// ! use compile_api_macros::api_endpoint;
// ! 
// ! #[api_endpoint]
// ! fn func(one: i32, two: i32, three: String) {
// !     let x = one + two;
// ! }
// ! ```
// ! This expands to:
// ! ```no_run
// ! pub async fn func(
// !     one: i32,
// !     two: i32,
// !     three: String,
// ! ) -> Result<actix_web::HttpResponse, utils::errors::NanoServiceError> {
// !     let x = one + two;
// ! }
// ! ```
// ! Notice:
// ! - The function is now `async` and `pub`
// ! - Return type is `Result<HttpResponse, NanoServiceError>`
// ! - Original arguments and body are preserved
// ! 
// ! ## Endpoint with DAL but no token
// ! If your endpoint requires database traits but you do not need token checks, specify `db_traits`:
// ! ```no_run
// ! use compile_api_macros::api_endpoint;
// ! 
// ! #[api_endpoint(db_traits=[One, Two, Three])]
 // ! fn some_func(one: i32, two: i32, three: String) {
// !     let x = one + two;
// ! }
// ! ```
// ! This expands to:
// ! ```no_run
// ! pub async fn some_func<X>(
// !     one: i32,
// !     two: i32,
// !     three: String,
// ! ) -> Result<actix_web::HttpResponse, utils::errors::NanoServiceError>
// ! where
// !     X: One + Two + Three,
// ! {
// !     let x = one + two;
// ! }
// ! ```
// ! The type parameter `X` is introduced, bounded by the traits you passed in.
// ! 
// ! ### Using multiple trait sets
// ! You can also include `email_traits` in the same endpoint:
// ! ```no_run
// ! #[api_endpoint(db_traits=[One, Two], email_traits=[EmailSender, EmailParser])]
 // ! fn combined_func(val: i32) {
// !     // Body can call logic requiring 'W' (email) or 'X' (db) 
// ! }
// ! ```
// ! This would expand to something like:
// ! ```no_run
// ! pub async fn combined_func<W, X>(
// !     val: i32,
// ! ) -> Result<actix_web::HttpResponse, utils::errors::NanoServiceError>
// ! where
// !     W: EmailSender + EmailParser,
// !     X: One + Two,
// ! {
// !     // function body
// ! }
// ! ```
// ! So you get distinct generic parameters for each set of traits.
// ! 
// ! ## Endpoint with DAL and token
// ! If your endpoint requires data access via traits and token-based checks, specify both:
// ! ```no_run
// ! #[api_endpoint(token=SuperAdminRoleCheck, db_traits=[One, Two, Three])]
 // ! fn another_func(one: i32, two: i32, three: String) {
// !     let x = one + two;
// ! }
// ! ```
// ! This expands to:
// ! ```no_run
// ! pub async fn another_func<X, Y, Z>(
// !     jwt: kernel::token::token::HeaderToken<Y, kernel::token::checks::SuperAdminRoleCheck>,
// !     one: i32,
// !     two: i32,
// !     three: String,
// ! ) -> Result<actix_web::HttpResponse, utils::errors::NanoServiceError>
// ! where
// !     X: One + Two + Three,
// !     Y: utils::config::GetConfigVariable + Send,
// !     Z: kernel::token::session_cache::traits::GetAuthCacheSession,
// ! {
// !     let user_session = match Z::get_auth_cache_session(&jwt).await {
// !         Ok(Some(session)) => session,
// !         Ok(None) => {
// !             return Err(
// !                 utils::errors::NanoServiceError::new(
// !                     "No longer in session cache".to_string(),
// !                     utils::errors::NanoServiceErrorStatus::Unauthorized,
// !                 ),
// !             );
// !         }
// !         Err(e) => return Err(e),
// !     };
// !     let x = one + two;
// ! }
// ! ```
// ! Here the `jwt` is passed in and the session is extracted from the cache. This means that on top
// ! of the `X` dal handle, the developer also has access to the `jwt` and the `user_session` extracted
// ! from the cache when using the macro.
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse::Parse, parse::ParseStream,
    ItemFn, Ident, Token, Result, bracketed, LitBool
};


// Struct to parse macro attributes
struct ApiEndpointArgs {
    token_type: Option<Ident>,
    db_traits: Vec<Ident>,
    email_traits: Vec<Ident>,
    env_variable_trait: bool,
}

impl Parse for ApiEndpointArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut token_type = None;
        let mut db_traits = Vec::new();
        let mut email_traits = Vec::new();
        let mut env_variable_trait = false;

        while !input.is_empty() {
            let key: Ident = input.parse()?; // Read key (e.g., "token" or "traits")
            input.parse::<Token![=]>()?; // Expect '='

            if key == "token" {
                // Read token type (e.g., "SomeThing")
                if input.peek(Ident) {
                    token_type = Some(input.parse()?);
                }
            } else if key == "db_traits" {
                // Read traits inside brackets `[Trait1, Trait2]`
                let content;
                bracketed!(content in input);
                while !content.is_empty() {
                    db_traits.push(content.parse()?); // Read each trait
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?; // Consume comma
                    }
                }
            } else if key == "email_traits" {
                // Read traits inside brackets `[Trait1, Trait2]`
                let content;
                bracketed!(content in input);
                while !content.is_empty() {
                    email_traits.push(content.parse()?); // Read each trait
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?; // Consume comma
                    }
                }
            } else if key == "env_variable_trait" {
                // Parse next token as a boolean literal
                let bool_lit: LitBool = input.parse()?;
                if bool_lit.value() {
                    env_variable_trait = bool_lit.value();
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?; // Consume comma between arguments
            }
        }

        Ok(ApiEndpointArgs { token_type, db_traits, email_traits, env_variable_trait })
    }
}

#[proc_macro_attribute]
pub fn api_endpoint(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ApiEndpointArgs { token_type, db_traits, email_traits, env_variable_trait } = parse_macro_input!(attr as ApiEndpointArgs);

    // define the status
    let mut token = false;

    // Parse the input function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract function components
    let fn_inputs = &input_fn.sig.inputs;
    let fn_body = &input_fn.block.stmts;
    let fn_name = &input_fn.sig.ident;

    let processed_inputs = match token_type.clone() {
        Some(token_type) => {
            token = true;
            quote! {
                jwt: kernel::token::token::HeaderToken<Y, kernel::token::checks::#token_type>, #fn_inputs
            }
        }
        None => {
            quote! {
                #fn_inputs
            }
        }
    };
    let session_call = match token_type {
        Some(_) => {
            quote! {
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
            }
        }
        None => {
            quote! {}
        }
    };


    let (email_trait_stub, email_trait_bounds) = if email_traits.is_empty() {
        (quote! { }, quote! { })
    } else {
        if db_traits.is_empty() {
            (quote! {W}, quote! { W: #(#email_traits)+* })
        } else {
            (quote! {W,}, quote! { W: #(#email_traits)+* , })
        }
    };

    let (dal_trait_stub, dal_trait_bounds) = if db_traits.is_empty() {
        (quote! { }, quote! { })
    } else {
        if token == false && env_variable_trait == false {
            (quote! {X}, quote! { X: #(#db_traits)+* })
        } else {
            (quote! {X,}, quote! { X: #(#db_traits)+* , })
        }
    };

    let (config_trait_stub, config_trait_bounds) = if token == false && env_variable_trait == false {
        (quote! { }, quote! { })
    } else {
        if token == false {
            (quote! {Y}, quote! { Y: utils::config::GetConfigVariable + Send })
        } else {
            (quote! {Y,}, quote! { Y: utils::config::GetConfigVariable + Send, })
        }
    };

    let (cache_trait_stub, cache_trait_bounds) = if token == false {
        (quote! { }, quote! { })
    } else {
        (quote! {Z}, quote! { Z: kernel::token::session_cache::traits::GetAuthCacheSession })
    };

    // Generate the expanded code
    let expanded = quote! {
        pub async fn #fn_name <#email_trait_stub #dal_trait_stub #config_trait_stub #cache_trait_stub>(
            #processed_inputs
        ) -> Result<actix_web::HttpResponse, utils::errors::NanoServiceError> 
        where
            #email_trait_bounds
            #dal_trait_bounds
            #config_trait_bounds
            #cache_trait_bounds
        {
            #session_call
            #(#fn_body)*
        }
    };
    TokenStream::from(expanded)
}
