use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Data, DeriveInput, Type, spanned::Spanned};

#[proc_macro_derive(FromArgumentMapping)]
pub fn from_argument_mapping_derive(input: TokenStream) -> TokenStream {
    let ast = match syn::parse(input) {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error().into(),
    };

    impl_from_argument_mapping(&ast)
}

fn impl_from_argument_mapping(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let data_struct = if let Data::Struct(data_struct) = &ast.data {
        data_struct
    } else {
        return quote_spanned! {
            ast.span() => compile_error!("expected struct")
        }
        .into();
    };

    let argument_builder = data_struct
        .fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().ok_or_else(|| {
                syn::Error::new(field.span(), "expected named field")
            })?;
            let field_name_str = field_name.to_string();
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

            let path_segment_ident = type_path
                .path
                .segments
                .first()
                .ok_or_else(|| {
                    syn::Error::new(field_type.span(), "invalid type path")
                })?
                .ident
                .to_string();

            let tokens = match path_segment_ident.as_str() {
                "Option" => quote! {
                    #field_name: arguments.optional(#field_name_str)?
                },
                "Vec" => quote! {
                    #field_name: arguments.multiple(#field_name_str)?
                },
                _ => quote! {
                    #field_name: arguments.required(#field_name_str)?
                },
            };

            Ok(tokens)
        })
        .collect::<Result<Vec<_>, syn::Error>>();

    let argument_builder = match argument_builder {
        Ok(vec) => vec,
        Err(err) => return err.to_compile_error().into(),
    };

    let generated = quote! {
                impl crate::FromArgumentMapping for #name<'static> {
        type Mapped<'__args> = #name<'__args>;

        fn from_argument_mapping<'__args>(
            arguments: &'__args crate::ArgumentMapping,
        ) -> Result<Self::Mapped<'__args>, crate::core::EvaluateNodeError> {
            Ok(#name {
                #(#argument_builder,)*
            })
        }
    }
            };

    generated.into()
}
