use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitStr, spanned::Spanned};

use crate::{DescriptionAttribute, crete_path::crate_path};

pub fn impl_constant_metadata(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &ast.ident;

    let name = ast
        .attrs
        .iter()
        .find_map(|attr| {
            if attr.path().is_ident("name") {
                Some(attr.parse_args::<LitStr>())
            } else {
                None
            }
        })
        .ok_or_else(|| {
            syn::Error::new(ast.span(), "Missing name attribute.")
        })??;

    let descriptions = ast
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("description") {
                Some(attr.parse_args::<DescriptionAttribute>())
            } else {
                None
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    let description_tokens = descriptions.iter().map(|attr| {
        let lang = &attr.language;
        let desc = &attr.description;

        quote! {
            message.add_message(#lang, Ok(#desc.to_string()));
        }
    });

    let evaluator_path = crate_path("evaluator")?;

    let generated = quote! {
        impl #evaluator_path::ConstantMetadata for #struct_name {

            fn constant_name() -> &'static str {
                #name
            }

            fn constant_description() -> translate_core::RenderedTranslatedMessage {
                let mut message = translate_core::RenderedTranslatedMessage::default();
                #(#description_tokens)*
                message
            }
        }
    };

    Ok(generated)
}
