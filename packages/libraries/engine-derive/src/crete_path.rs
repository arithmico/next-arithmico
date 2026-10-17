use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn crate_path(name: &str) -> syn::Result<TokenStream> {
    let crate_name = crate_name(name).map_err(|err| {
        syn::Error::new(
            Span::call_site(),
            format!("Failed to lookup crate name: {}", err),
        )
    })?;
    Ok(match crate_name {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(::#ident)
        }
    })
}
