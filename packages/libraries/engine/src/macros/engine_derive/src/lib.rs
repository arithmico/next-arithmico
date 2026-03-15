use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Type};

#[proc_macro_derive(FromArgumentMapping)]
pub fn from_argument_mapping_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("should parse");

    impl_from_argument_mapping(&ast)
}

fn impl_from_argument_mapping(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data_struct = if let Data::Struct(data_struct) = &ast.data {
        data_struct
    } else {
        panic!("only struct supported")
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
            panic!("type path does not exist")
        }
    });

    let generated = quote! {
        impl crate::FromArgumentMapping for #name {
            fn from_argument_mapping(
                arguments: &crate::ArgumentMapping,
            ) -> Result<Self, crate::core::EvaluateNodeError> {
                Ok(Self {
                    #(#argument_builder,)*
                })
            }
        }
    };
    generated.into()
}
