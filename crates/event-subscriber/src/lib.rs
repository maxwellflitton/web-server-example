extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, format_ident, ToTokens};
use syn::{
    parse_macro_input, FnArg, PatType, ItemFn,
    spanned::Spanned
};


#[proc_macro_attribute]
pub fn subscribe_to_event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    // Get the function name
    let func_name = &input_fn.sig.ident;
    
    // Generate new function names
    let register_func_name = format_ident!("register_{}", func_name);
    let init_func_name = format_ident!("init_{}", func_name);
    let routed_func_name = format_ident!("routed_{}", func_name);
    let check_func_name = format_ident!("_check_{}", func_name);

    // Ensure the function is async
    if input_fn.sig.asyncness.is_none() {
        return syn::Error::new(input_fn.sig.asyncness.span(), "Function must be async")
            .to_compile_error()
            .into();
    }

    // Ensure the function has exactly one parameter
    if input_fn.sig.inputs.len() != 1 {
        return syn::Error::new(
            input_fn.sig.inputs.span(), 
            "Function must have exactly one parameter which is the message struct that it is subscribing to"
        )
            .to_compile_error()
            .into();
    }

    // Get the first argument (if any)
    let first_param = input_fn.sig.inputs.first().expect("Function must have at least one parameter");

    // Ensure the first parameter is a typed argument
    let param_type = if let FnArg::Typed(PatType { ty, .. }) = first_param {
        ty
    } else {
        return syn::Error::new(first_param.span(), "Expected a typed parameter")
            .to_compile_error()
            .into();
    };
    let param_name = param_type.to_token_stream().to_string().trim_matches('"').to_string();

    // Generate trait-bound verification code
    let check_traits = quote! {
        #[doc(hidden)]
        fn #check_func_name<T>()
        where
            T: serde::Serialize + serde::de::DeserializeOwned,
        {}
        #[doc(hidden)]
        const _: fn() = || {
            #check_func_name::<#param_type>();
        };
    };


    // Generate the expanded code
    let expanded = quote! {

        #input_fn

        // Inline trait checks
        #check_traits

        // Define a router function that accepts bincode and returns a boxed future
        #[doc(hidden)]
        fn #routed_func_name(data: Vec<u8>) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
            std::boxed::Box::pin(async move {
                let deserialized: #param_type = nanoservices_utils::bincode::deserialize(&data).unwrap();
                #func_name(deserialized).await;
            })
        }
    
        // Register function
        #[doc(hidden)]
        fn #register_func_name() {
            crate::tokio_event_adapter_runtime::insert_into_hashmap(
                #param_name.to_string(),
                #routed_func_name
            );
        }

        // Init function
        #[nanoservices_utils::ctor::ctor]
        fn #init_func_name() {
            println!("Initializing function: {}", stringify!(#func_name));
            #register_func_name();
        }
    };

    TokenStream::from(expanded)
}
