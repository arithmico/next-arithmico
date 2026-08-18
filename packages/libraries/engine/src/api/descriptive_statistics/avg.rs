use common::Language;
use engine_derive::FunctionArguments;
use evaluator::{Error, FunctionEndpoint, Options};
use math_utils::calculate_average;
use node::Number;
use trace::{CombineHulls, TracableMut};

#[derive(FunctionArguments)]
#[name("avg")]
#[description(
    Language::German,
    "Berechnet das arithmetische Mittel aus den übergebenen Werten."
)]
#[description(
    Language::English,
    "Calculates the arithmetic mean of the given arguments."
)]
pub struct AvgArgs<'a> {
    #[description(Language::German, "Werte")]
    #[description(Language::English, "values")]
    x: Vec<&'a Number>,
}

pub struct AvgEndpoint;

impl FunctionEndpoint for AvgEndpoint {
    type Output = Number;
    type Arguments<'a> = AvgArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        AvgArgs { x }: Self::Arguments<'a>,
        _options: Options,
    ) -> Result<Self::Output, Error> {
        let value = match &x[..] {
            [] => return Err(Error::missing_parameter("x")),
            [item] => item.value,
            items => {
                let values =
                    items.iter().map(|node| node.value).collect::<Vec<_>>();

                calculate_average(&values).ok_or_else(|| {
                    Error::unreachable().with_optional_span(x.combine_hulls())
                })?
            }
        };

        Ok(Number::new(value))
    }
}
