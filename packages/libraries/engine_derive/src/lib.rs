use proc_macro::TokenStream;
use syn::{
    LitStr, Path, Token,
    parse::{Parse, ParseStream},
};

use crate::{
    constant_metadata::impl_constant_metadata,
    function_arguments::impl_function_arguments,
};

mod constant_metadata;
mod function_arguments;

struct DescriptionAttribute {
    language: Path,
    description: LitStr,
}

impl Parse for DescriptionAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let language: Path = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let description: LitStr = input.parse()?;

        Ok(Self {
            language,
            description,
        })
    }
}

#[proc_macro_derive(
    FunctionArguments,
    attributes(description, name, skip_evaluate, default)
)]
pub fn function_arguments_derive(input: TokenStream) -> TokenStream {
    let ast = match syn::parse(input) {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error().into(),
    };

    impl_function_arguments(&ast)
}

#[proc_macro_derive(ConstantMetadata, attributes(name, description))]
pub fn constant_endpoint_derive(input: TokenStream) -> TokenStream {
    let ast = match syn::parse(input) {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error().into(),
    };

    impl_constant_metadata(&ast)
}
