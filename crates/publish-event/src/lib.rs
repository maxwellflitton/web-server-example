extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};


#[proc_macro]
pub fn publish_event(input: TokenStream) -> TokenStream {
    let instance_name = parse_macro_input!(input as Ident);

    let expanded = quote! {
        {
            let type_name = std::any::type_name_of_val(&#instance_name);
            let name = type_name.split("::").last().unwrap(); // Extract the last segment (e.g., "AddNumbers")
            let data = bincode::serialize(&#instance_name).unwrap();
            crate::tokio_event_adapter_runtime::publish_event(name, data);
        }
    };

    TokenStream::from(expanded)
}
