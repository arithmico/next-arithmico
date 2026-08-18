use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Data, DeriveInput, Ident, LitStr, Type, spanned::Spanned};

use crate::{DescriptionAttribute, crete_path::crate_path};

struct ArgumentField {
    ident: Ident,
    name: String,
    cardinality: ArgumentCardinality,
    field_type: Type,
    description: Vec<proc_macro2::TokenStream>,
    skip_evaluate: bool,
}

enum ArgumentCardinality {
    Optional,
    OptionalWithDefault { default: proc_macro2::TokenStream },
    Required,
    Multiple { min: usize, max: Option<usize> },
}

fn parse_repeatable_attribute(
    attribute: &syn::Attribute,
) -> syn::Result<(usize, Option<usize>)> {
    let mut min = 0;
    let mut max = None;

    attribute.parse_nested_meta(|meta| {
        if meta.path.is_ident("min") {
            let value = meta.value()?;
            min = value.parse::<syn::LitInt>()?.base10_parse()?;
            return Ok(());
        }

        if meta.path.is_ident("max") {
            let value = meta.value()?;
            max = Some(value.parse::<syn::LitInt>()?.base10_parse()?);
            return Ok(());
        }

        Err(meta.error("expected `min` or `max`"))
    })?;

    if let Some(max) = max {
        if min > max {
            return Err(syn::Error::new_spanned(
                attribute,
                "`repeatable` requires `min <= max`",
            ));
        }
    }

    Ok((min, max))
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
                description.add_message(#language, Ok(#description.to_string()));
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

            let repeatable = field
                .attrs
                .iter()
                .find(|attribute| attribute.path().is_ident("repeatable"))
                .map(parse_repeatable_attribute)
                .transpose()?;

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

            if repeatable.is_some() && root_field_type != "Vec" {
                return Err(syn::Error::new(
                    field.span(),
                    "`repeatable` can only be used with Vec arguments",
                ));
            }

            let cardinality = if let Some(default) = default {
                ArgumentCardinality::OptionalWithDefault { default }
            } else {
                match root_field_type.as_str() {
                    "Option" => ArgumentCardinality::Optional,
                    "Vec" => {
                        let (min, max) = repeatable.unwrap_or((0, None));

                        ArgumentCardinality::Multiple { min, max }
                    }
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
                    ArgumentCardinality::Multiple { .. } => quote! {
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
                    ArgumentCardinality::Multiple { min, max } => {
                        let max = match max {
                            Some(max) => quote! { Some(#max) },
                            None => quote! { None },
                        };

                        quote! {
                        .argument(
                            #name,
                            |arg| arg.repeatable(#min, #max).node_type(
                                <#field_type as node::GetStaticNodeType>::static_node_type()
                            )
                            #(#description)*
                            #evaluate
                        )
                    }
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

    let evaluator_path = crate_path("evaluator");

    let generated = quote! {
        impl<'a> #evaluator_path::FunctionArguments<'a> for #struct_name<'a> {
            fn from_mapping(
                arguments: &'a #evaluator_path::ArgumentMapping,
            ) -> Result<Self, #evaluator_path::Error> {
                Ok(#struct_name {
                    #(#field_mapping,)*
                })
            }

            fn signature() -> node::FunctionSignature {
                node::FunctionSignature::new()
                #(#signature_arguments)*
            }

            fn function_description() -> translate_core::RenderedTranslatedMessage {
                let mut description = translate_core::RenderedTranslatedMessage::new();
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
