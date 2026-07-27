use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{DeriveInput, LitStr, spanned::Spanned};

use crate::DescriptionAttribute;

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
            map.insert(#lang, #desc.to_string());
        }
    });

    let generated = quote! {
            impl crate::core::ConstantMetadata for #struct_name {
        fn constant_name() -> &'static str {
            #name
        }

        fn constant_description() -> crate::core::TranslatedString {
            let mut map = std::collections::HashMap::new();
            #(#description_tokens)*
            map
        }
    }
        };

    generated.into()
}
