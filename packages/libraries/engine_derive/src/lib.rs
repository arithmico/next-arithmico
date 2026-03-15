use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Data, DeriveInput, Type, spanned::Spanned};

#[proc_macro_derive(FromArgumentsBinding)]
pub fn from_arguments_binding_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("should parse");

    impl_from_arguments_binding(&ast)
}

fn impl_from_arguments_binding(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data_struct = if let Data::Struct(data_struct) = &ast.data {
        data_struct
    } else {
        return quote_spanned! {
            ast.span() => compile_error!("expected struct")
        }.into();
    };

    let argument_builder = data_struct.fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_str = field_name.to_string();
        let field_type = &field.ty;

        if let Type::Path(type_path) = field_type {
            let path_segment_ident =
                type_path.path.segments.last().unwrap().ident.to_string();

            match path_segment_ident.as_str() {
                "Option" => quote! {
                    #field_name: arguments.optional(#field_str)?
                },
                "Vec" => quote! {
                    #field_name: arguments.multiple(#field_str)?
                },
                _ => quote! {
                    #field_name: arguments.required(#field_str)?
                },
            }
        } else {
            return quote_spanned! {
                field_type.span() => compile_error!("type path should begin with Option, Vec or raw type")
            }.into();
        }
    });

    let generated = quote! {
        impl crate::FromArgumentsBinding for #name {
            fn from_arguments_binding(
                arguments: &crate::ArgumentsBinding,
            ) -> Result<Self, crate::core::EvaluateNodeError> {
                Ok(Self {
                    #(#argument_builder,)*
                })
            }
        }
    };
    generated.into()
}
