use std::f64::consts::PI;

use engine_derive::FromArgumentMapping;
use node::{FunctionSignature, Node, NodeType, Number};

use crate::{
    api::global_utils::F64Extension,
    core::{
        EndpointBuilder, EvaluateNodeError, FunctionSignature, HostEndpoint,
        Language, NodeType, Number,
    },
    function_executor_wrapper, Context, Node,
};

#[derive(FromArgumentMapping)]
struct SinArgs<'a> {
    x: &'a Number,
}

fn sin_executor(
    SinArgs { x }: SinArgs,
    _context: &Context,
) -> Result<Node, EvaluateNodeError> {
    let value = x.value;

    if value.is_close_to_multiple_of(PI) {
        return Ok(Number::new(0.0));
    }

    Ok(Number::new(value.sin()))
}

#[cfg(feature = "api_endpoint_trigonometry_sin")]
pub fn load_sin_endpoint(builder: EndpointBuilder) -> HostEndpoint {
    let signature = FunctionSignature::new()
        .argument("x", |argument| {
            argument
                .evaluate()
                .description(Language::English, "angle")
                .description(Language::German, "Winkel")
                .node_type(NodeType::Number)
        })
        .add_return_type(NodeType::Number);

    builder
        .name("sin")
        .description(Language::English, "Calculate the sine of x.")
        .description(Language::German, "Berechnet den Sinus von x.")
        .function(signature)
        .executor(function_executor_wrapper!(SinArgs, sin_executor))
}
