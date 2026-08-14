use std::ops::Deref;

use common::Language;
use engine_derive::FunctionArguments;

use evaluator::{
    Error, ErrorKind, EvaluateNode, FunctionEndpoint, MapToEvaluatorError,
    Options,
};
use node::{Function, Node, Number, Symbol, Tensor};
use node_validator::{FunctionValidator, NodeValidator};
use translate::TranslatedMessage;

use crate::translation_provider::translation_resolver;

#[derive(FunctionArguments)]
#[name("table")]
#[description(
    Language::German,
    "Erstellt zu der Funktion f eine Wertetabelle mit Wertepaaren [x; f(x)] für x-Werte aus dem Intervall [start; stop]. "
)]
#[description(
    Language::English,
    "Maps f(x) to [x, f(x)] within the given interval [start, stop]."
)]
pub struct TableArgs<'a> {
    #[description(Language::German, "Funktion")]
    #[description(Language::English, "function")]
    f: &'a Function,

    #[description(Language::German, "Startwert")]
    #[description(Language::English, "start value")]
    start: &'a Number,

    #[description(Language::German, "Endwert")]
    #[description(Language::English, "stop value")]
    stop: &'a Number,
    // TODO: add step parameter
}

pub struct TableEndpoint;

impl FunctionEndpoint for TableEndpoint {
    type Output = Tensor;
    type Arguments<'a> = TableArgs<'a>;

    // TODO: unit tests
    fn executor<'a>(
        TableArgs { f, start, stop }: Self::Arguments<'a>,
        options: Options,
    ) -> Result<Self::Output, Error> {
        f.validate_argument_count(1)
            .map_to_error_kind(ErrorKind::InvalidParameterValue)?;

        let variable_name = f
            .signature
            .argument_names()
            .first()
            .copied()
            .ok_or_else(|| Error::unreachable())?;

        f.expression
            .validate_unknown_symbol_name(&options.stack.names(), variable_name)
            .map_to_error_kind(ErrorKind::RuntimeError)?;

        if start.value >= stop.value {
            return Err(Error::runtime_error(TranslatedMessage::new(
                "api.table.start_must_be_greater_than_stop",
                translation_resolver,
            )));
        }

        let n = (start.value - stop.value).abs().floor() as usize;
        let step: f64 = 1.0;
        let mut current = start.value;
        let mut rows = Vec::<Node>::with_capacity(n + 1);
        let mut stack = options.stack.clone();
        rows.push(Tensor::new_node(vec![
            Symbol::new(variable_name),
            f.expression.deref().clone(),
        ]));

        while current <= stop.value {
            let x = Number::new_node(current);
            stack.insert(variable_name, x.clone());
            let y = f.expression.evaluate(Options {
                stack: &stack,
                ..options
            })?;
            rows.push(Tensor::new_node(vec![x, y]));
            current += step;
        }

        Ok(Tensor::new(rows))
    }
}
