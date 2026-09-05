use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{
    Error, ErrorKind, FunctionEndpoint, MapToEvaluatorError, Options,
};
use math_utils::calculate_permutations;
use node::Number;
use validator::NumberValidator;

#[derive(FunctionArguments)]
#[name("perm")]
#[description(
    Language::German,
    "Berechnet die Anzahl der Permutationen von k Elementen aus n Elementen ohne Zurücklegen. Auch als nPr bekannt."
)]
#[description(
    Language::English,
    "Computes the number of permutations of k elements chosen from n elements without replacement. Also known as nPr."
)]
pub struct PermArgs<'a> {
    #[description(Language::German, "Anzahl der verfügbaren Elemente")]
    #[description(Language::English, "Number of available elements")]
    n: &'a Number,

    #[description(Language::German, "Anzahl der anzuordnenden Elemente")]
    #[description(Language::English, "Number of elements to arrange")]
    k: &'a Number,
}

pub struct PermEndpoint;

impl FunctionEndpoint for PermEndpoint {
    type Output = Number;
    type Arguments<'a> = PermArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        PermArgs { n, k }: Self::Arguments<'a>,
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

        Ok(Number::new(calculate_permutations(n_value, k_value)))
    }
}
