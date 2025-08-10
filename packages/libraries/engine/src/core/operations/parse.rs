use crate::core::{
    operations::parse::parenthesis_check::check_missing_open_parenthesis,
    Context, DecimalFormat, Definition, Node, ParseNodeError,
};
use nom::{
    combinator::{all_consuming, cut},
    IResult, Parser,
};

mod cache;
mod literal;
mod parenthesis_check;
mod relation;
mod sub_expression;
mod tag;
mod trace;
mod with_parser;

pub use cache::*;
pub use literal::*;
pub use relation::*;
pub use sub_expression::*;
pub use tag::*;
pub use trace::*;
pub use with_parser::*;

pub type ParseResult<'a> = IResult<&'a str, Node, ParseNodeError>;

pub trait ParseNode {
    fn parse(input: &'_ str) -> ParseResult<'_>;
}

pub fn parse(input: &str, context: &Context) -> Result<Node, ParseNodeError> {
    let input = match context.decimal_format {
        DecimalFormat::Comma => input.replace(",", ".").replace(";", ","),
        DecimalFormat::Dot => input.to_string(),
    };

    check_missing_open_parenthesis(&input)?;

    let result = all_consuming(cut(Definition::parse))
        .parse(&input)
        .map(|(_, node)| node)
        .map_err(|error| match error {
            nom::Err::Incomplete(_) => {
                unreachable!("incomplete parser error")
            }
            nom::Err::Error(error) => error,
            nom::Err::Failure(error) => error,
        });

    clear_cache();

    result
}
