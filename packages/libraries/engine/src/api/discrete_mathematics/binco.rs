use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::calculate_combinations;
use node::Number;
use validator::NumberValidator;

#[derive(FunctionArguments)]
#[name("binco")]
#[description(
    Language::German,
    "Berechnet den Binomialkoeffizienten n über k, auch als nCr bekannt."
)]
#[description(
    Language::English,
    "Computes the binomial coefficient n choose k. Also known as nCr."
)]
pub struct BincoArgs<'a> {
    #[description(Language::German, "Anzahl der verfügbaren Elemente")]
    #[description(Language::English, "Number of available elements")]
    n: &'a Number,

    #[description(Language::German, "Anzahl der auszuwählenden Elemente")]
    #[description(Language::English, "Number of elements to choose")]
    k: &'a Number,
}

pub struct BincoEndpoint;

impl FunctionEndpoint for BincoEndpoint {
    type Output = Number;
    type Arguments<'a> = BincoArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        BincoArgs { n, k }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        let n_value = n
            .validate_positive()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .value as usize;
        let k_value = k
            .validate_positive()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_less_than_or_equal(n.value)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .validate_integer()
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?
            .value as usize;

        Ok(Number::new(calculate_combinations(n_value, k_value)))
    }
}
