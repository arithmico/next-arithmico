use translate_core::Language;

use crate::{
    core::{EndpointBuilder, HostEndpoint, NodeType},
    FunctionSignature,
};

pub fn load_echo_endpoint(builder: EndpointBuilder) -> HostEndpoint {
    let signature = FunctionSignature::new()
        .argument("x", |argument| {
            argument
                .description(Language::English, "Value")
                .description(Language::German, "Wert")
                .node_type(NodeType::Any)
        })
        .add_return_type(NodeType::Any);

    builder
        .description(Language::English, "Repeat x")
        .description(Language::German, "Wiederhole x")
        .function(signature)
        .executor(|arguments, _context| {
            let argument = arguments.get_parameter_value("x")?;
            Ok(argument)
        })
}
