use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Data, DeriveInput, Ident, LitStr, Type, spanned::Spanned};

use crate::DescriptionAttribute;

struct ArgumentField {
    ident: Ident,
    name: String,
    cardinality: ArgumentCardinality,
    field_type: Type,
    description: Vec<proc_macro2::TokenStream>,
    skip_evaluate: bool,
}

enum ArgumentCardinality {
    Multiple,
    Optional,
    OptionalWithDefault { default: proc_macro2::TokenStream },
    Required,
}

pub(crate) fn impl_function_arguments(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let data_struct = if let Data::Struct(data_struct) = &ast.data {
        data_struct
    } else {
        return quote_spanned! {
            ast.span() => compile_error!("expected struct")
        }
        .into();
    };

    let Some(function_name) = ast.attrs.iter().find_map(|attribute| {
        if attribute.path().is_ident("name") {
            Some(attribute.parse_args::<LitStr>())
        } else {
            None
        }
    }) else {
        return quote_spanned! {
            ast.span() => compile_error!("missing function name attribute")
        }
        .into();
    };

    let function_name = match function_name {
        Ok(v) => v,
        Err(err) => return err.to_compile_error().into(),
    };

    let struct_descriptions = ast
        .attrs
        .iter()
        .filter_map(|attribute| {
            if attribute.path().is_ident("description") {
                Some(attribute.parse_args::<DescriptionAttribute>())
            } else {
                None
            }
        })
        .collect::<Result<Vec<_>, _>>();

    let struct_attributes = match struct_descriptions {
        Ok(vec) => vec,
        Err(e) => return e.to_compile_error().into(),
    };

    let struct_descriptions = struct_attributes
        .into_iter()
        .map(|attribute| {
            let language = &attribute.language;
            let description = &attribute.description;
            quote! {
                description.insert(#language, #description.to_string());
            }
        })
        .collect::<Vec<_>>();

    let fields = data_struct
        .fields
        .iter()
        .map(|field| {
            let ident = field.ident.clone().ok_or_else(|| {
                syn::Error::new(field.span(), "expected named field")
            })?;

            let skip_evaluate = field
                .attrs
                .iter()
                .find(|attribute| attribute.path().is_ident("skip_evaluate"))
                .is_some();

            let default = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("default"))
                .map(|attr| {
                    let expr: syn::Expr = attr.parse_args().unwrap();
                    quote! { #expr }
                });

            let description = field
                .attrs
                .iter()
                .filter_map(|attribute| {
                    if attribute.path().is_ident("description") {
                        Some(attribute.parse_args::<DescriptionAttribute>())
                    } else {
                        None
                    }
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|attribute| {
                    let language = &attribute.language;
                    let description = &attribute.description;
                    quote! {
                        .description(
                            #language,
                            #description
                        )
                    }
                })
                .collect::<Vec<_>>();

            let name = ident.to_string();
            let field_type = match &field.ty {
                Type::Reference(type_ref) => &*type_ref.elem,
                other => other,
            };

            let type_path = match field_type {
                Type::Path(path) => Ok(path),
                _ => Err(syn::Error::new(
                    field_type.span(),
                    "type path should begin with Option, Vec or raw type",
                )),
            }?;

            let root_field_type = type_path
                .path
                .segments
                .first()
                .ok_or_else(|| {
                    syn::Error::new(field_type.span(), "invalid type path")
                })?
                .ident
                .to_string();

            let cardinality = if let Some(default) = default {
                ArgumentCardinality::OptionalWithDefault { default }
            } else {
                match root_field_type.as_str() {
                    "Option" => ArgumentCardinality::Optional,
                    "Vec" => ArgumentCardinality::Multiple,
                    _ => ArgumentCardinality::Required,
                }
            };

            Ok(ArgumentField {
                ident,
                name,
                cardinality,
                field_type: field_type.clone(),
                description,
                skip_evaluate,
            })
        })
        .collect::<Result<Vec<_>, syn::Error>>();

    let fields = match fields {
        Ok(vec) => vec,
        Err(err) => return err.to_compile_error().into(),
    };

    let field_mapping = fields
        .iter()
        .map(
            |ArgumentField {
                 ident,
                 name,
                 cardinality,
                 ..
             }| {
                match cardinality {
                    ArgumentCardinality::Optional => quote! {
                        #ident: arguments.optional(#name)?
                    },
                    ArgumentCardinality::Multiple => quote! {
                        #ident: arguments.multiple(#name)?
                    },
                    ArgumentCardinality::Required => quote! {
                        #ident: arguments.required(#name)?
                    },
                    ArgumentCardinality::OptionalWithDefault { default} => quote! {
                        #ident: arguments.optional_with_default(#name, #default)?
                    }
                }
            },
        )
        .collect::<Vec<_>>();

    let signature_arguments = fields
        .iter()
        .map(
            |ArgumentField {
                 name,
                 cardinality,
                 field_type,
                 description,
                 skip_evaluate,
                 ..
             }| {
                let evaluate = if !*skip_evaluate {
                    Some(quote! {.evaluate()})
                } else {
                    None
                };

                match cardinality {
                    ArgumentCardinality::Optional => quote! {
                        .argument(
                            #name,
                            |arg| arg.optional().node_type(
                                <#field_type as node::GetStaticNodeType>::static_node_type()
                            )
                            #(#description)*
                            #evaluate
                        )
                    },
                    ArgumentCardinality::Multiple => quote! {
                        .argument(
                            #name,
                            |arg| arg.repeatable().node_type(
                                <#field_type as node::GetStaticNodeType>::static_node_type()
                            )
                            #(#description)*
                            #evaluate
                        )
                    },
                    ArgumentCardinality::Required => quote! {
                        .argument(
                            #name,
                            |arg| arg.node_type(
                                <#field_type as node::GetStaticNodeType>::static_node_type()
                            )
                            #(#description)*
                            #evaluate
                        )
                    },
                    ArgumentCardinality::OptionalWithDefault { default } => quote! {
                        .argument(
                            #name,
                            |arg| arg
                                .default((#default).clone())
                                .node_type(
                                    <#field_type as node::GetStaticNodeType>::static_node_type()
                                )
                            #(#description)*
                            #evaluate
                        )
                    }
                }
            },
        )
        .collect::<Vec<_>>();

    let generated = quote! {
        impl<'a> crate::FunctionArguments<'a> for #struct_name<'a> {
            fn from_mapping(
                arguments: &'a crate::ArgumentMapping,
            ) -> Result<Self, crate::core::EvaluateNodeError> {
                Ok(#struct_name {
                    #(#field_mapping,)*
                })
            }

            fn signature() -> node::FunctionSignature {
                node::FunctionSignature::new()
                #(#signature_arguments)*
            }

            fn function_description() -> crate::core::TranslatedString {
                let mut description = std::collections::HashMap::new();
                #(
                    #struct_descriptions
                )*
                description
            }

            fn function_name() -> &'static str {
                #function_name
            }
        }
    };

    generated.into()
}
