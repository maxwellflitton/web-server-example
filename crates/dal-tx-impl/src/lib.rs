extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse::Parse, parse::ParseStream,
    ItemFn, Ident, Token, Result
};


struct ImplementTraitArgs {
    struct_name: Ident,
    trait_name: Ident,
    fn_name: Ident,
}

impl Parse for ImplementTraitArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let struct_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let trait_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let fn_name: Ident = input.parse()?;
        Ok(Self {
            struct_name,
            trait_name,
            fn_name,
        })
    }
}

#[proc_macro_attribute]
pub fn impl_transaction(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute arguments
    let ImplementTraitArgs {
        struct_name,
        trait_name,
        fn_name,
    } = parse_macro_input!(attr as ImplementTraitArgs);

    // Parse the input function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract function components
    let fn_inputs = &input_fn.sig.inputs;
    let fn_body = &input_fn.block;

    // Extract the function signature generics is there are any
    let fn_generics = &input_fn.sig.generics;

    let fn_output = match &input_fn.sig.output {
        syn::ReturnType::Type(_, ty) => ty.as_ref(),
        syn::ReturnType::Default => {
            panic!("Function must have a return type.")
        }
    };

    // Generate the expanded code
    let expanded = quote! {
        impl #trait_name for #struct_name {
            fn #fn_name #fn_generics (#fn_inputs) -> impl std::future::Future<Output = #fn_output> + Send {
                async move #fn_body
            }
        }
    };
    TokenStream::from(expanded)
}
