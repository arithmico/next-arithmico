use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{DeriveInput, LitStr, spanned::Spanned};

use crate::{DescriptionAttribute, crete_path::crate_path};

pub fn impl_constant_metadata(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let Some(name_attr) = ast.attrs.iter().find_map(|attr| {
        if attr.path().is_ident("name") {
            Some(attr.parse_args::<LitStr>())
        } else {
            None
        }
    }) else {
        return quote_spanned! {
            ast.span() => compile_error!("missing #[name(...)] attribute")
        }
        .into();
    };

    let name = match name_attr {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

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
        .collect::<Result<Vec<_>, _>>();

    let descriptions = match descriptions {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let description_tokens = descriptions.iter().map(|attr| {
        let lang = &attr.language;
        let desc = &attr.description;

        quote! {
            message.add_message(#lang, Ok(#desc.to_string()));
        }
    });

    let evaluator_path = crate_path("evaluator");

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

    generated.into()
}
