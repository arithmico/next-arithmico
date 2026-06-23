use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DeriveInput, Ident, Type};

struct ArgumentField {
    ident: Ident,
    name: String,
    cardinality: ArgumentCardinality,
    field_type: Type,
}

enum ArgumentCardinality {
    Multiple,
    Optional,
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

    let fields = data_struct
        .fields
        .iter()
        .map(|field| {
            let ident = field.ident.clone().ok_or_else(|| {
                syn::Error::new(field.span(), "expected named field")
            })?;
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

            let cardinality = match root_field_type.as_str() {
                "Option" => ArgumentCardinality::Optional,
                "Vec" => ArgumentCardinality::Multiple,
                _ => ArgumentCardinality::Required,
            };

            Ok(ArgumentField {
                ident,
                name,
                cardinality,
                field_type: field_type.clone(),
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
                }
            },
        )
        .collect::<Vec<_>>();

    let signature_arguments = fields
        .iter()
        .map(
            |ArgumentField {
                 ident,
                 name,
                 cardinality,
                 field_type,
             }| {
                match cardinality {
                    ArgumentCardinality::Optional => quote! {
                        .argument(
                            #name,
                            |arg| arg.optional().node_type(
                                <#field_type as node::GetStaticNodeType>::static_node_type()
                            )
                        )
                    },
                    ArgumentCardinality::Multiple => quote! {
                        .argument(
                            #name,
                            |arg| arg.repeatable().node_type(
                                <#field_type as node::GetStaticNodeType>::static_node_type()
                            )
                        )
                    },
                    ArgumentCardinality::Required => quote! {
                        .argument(
                            #name,
                            |arg| arg.node_type(
                                <#field_type as node::GetStaticNodeType>::static_node_type()
                            )
                        )
                    },
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
        }
    };

    generated.into()
}
