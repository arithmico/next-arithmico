use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::factorize;
use node::{Number, Power, Product};
use validator::NumberValidator;

#[derive(FunctionArguments)]
#[name("fact")]
#[description(
    Language::German,
    "Faktorisiert die übergebene Zahl in seine Primfaktoren."
)]
#[description(
    Language::English,
    "Factorizes the given number in its prime factors."
)]
pub struct FactArgs<'a> {
    #[description(Language::German, "Wert")]
    #[description(Language::English, "value")]
    n: &'a Number,
}

pub struct FactEndpoint;

impl FunctionEndpoint for FactEndpoint {
    type Output = Product;
    type Arguments<'a> = FactArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        FactArgs { n }: Self::Arguments<'a>,
        _context: Options,
    ) -> Result<Self::Output, Error> {
        let n_value = n
            .validate_greater_than_or_equal(0.0)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .value as u64;

        let elements = factorize(n_value)
            .into_iter()
            .map(|(base, exponent)| {
                if exponent == 1 {
                    Number::new_node(base as f64)
                } else {
                    Power::new(
                        Number::new_node(base as f64),
                        Number::new_node(exponent as f64),
                    )
                }
            })
            .collect::<Vec<_>>();

        Ok(Product::new(elements))
    }
}
