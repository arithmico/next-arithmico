use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn crate_path(name: &str) -> TokenStream {
    match crate_name(name) {
        Ok(found_crate) => match found_crate {
            FoundCrate::Itself => quote!(crate),
            FoundCrate::Name(name) => {
                let ident = Ident::new(&name, Span::call_site());
                quote!(::#ident)
            }
        },
        Err(err) => {
            let err = err.to_string();
            quote!(compile_error!(#err))
        }
    }
}
