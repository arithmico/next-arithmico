use std::collections::HashMap;

use node::{
    FunctionSignature, GetStaticNodeType, IntoNode, Node, NodeType, Number,
};
use translate_core::Language;

use crate::{
    core::EvaluateNodeError, ArgumentMapping, Context, FunctionArguments,
};

pub type FunctionExecutor =
    fn(&ArgumentMapping, &Context) -> Result<Node, EvaluateNodeError>;

pub type ConstantExecutor = fn(&Context) -> Node;

pub type TranslatedString = HashMap<Language, String>;

#[derive(Debug)]
pub struct EndpointMetadata {
    endpoint_name: String,
    module_id: String,
    module_name: TranslatedString,
    description: TranslatedString,
}

impl EndpointMetadata {
    pub fn new(
        endpoint_name: String,
        module_id: String,
        module_name: TranslatedString,
        description: TranslatedString,
    ) -> Self {
        EndpointMetadata {
            endpoint_name,
            module_id,
            module_name,
            description,
        }
    }

    pub fn endpoint_name(&self) -> &str {
        &self.endpoint_name
    }

    pub fn module_id(&self) -> &str {
        &self.module_id
    }

    pub fn module_name(&self) -> &TranslatedString {
        &self.module_name
    }

    pub fn description(&self) -> &TranslatedString {
        &self.description
    }
}

#[derive(Debug)]
pub enum HostEndpoint {
    Function {
        metadata: EndpointMetadata,
        signature: FunctionSignature,
        executor: FunctionExecutor,
    },
    Constant {
        metadata: EndpointMetadata,
        executor: ConstantExecutor,
    },
}

impl HostEndpoint {
    pub fn function<F: FunctionEndpoint>(
        module_id: &str,
        module_name: TranslatedString,
    ) -> Self {
        Self::Function {
            metadata: EndpointMetadata {
                endpoint_name: F::name().to_string(),
                module_id: module_id.to_string(),
                module_name,
                description: F::description(),
            },
            signature: F::signature(),
            executor: F::call,
        }
    }

    pub fn constant<C: ConstantEndpoint>(
        module_id: &str,
        module_name: TranslatedString,
    ) -> Self {
        Self::Constant {
            metadata: EndpointMetadata {
                endpoint_name: C::name().to_string(),
                module_id: module_id.to_string(),
                module_name,
                description: C::description(),
            },
            executor: C::call,
        }
    }

    fn metadata(&self) -> &EndpointMetadata {
        match self {
            HostEndpoint::Function { metadata, .. } => metadata,
            HostEndpoint::Constant { metadata, .. } => metadata,
        }
    }

    pub fn endpoint_name(&self) -> &str {
        &self.metadata().endpoint_name
    }

    pub fn module_id(&self) -> &str {
        &self.metadata().module_id
    }

    pub fn module_name(&self) -> &TranslatedString {
        &self.metadata().module_name
    }
}

pub trait FunctionEndpoint {
    type Output: GetStaticNodeType + IntoNode;
    type Arguments<'a>: FunctionArguments<'a>;

    fn name() -> &'static str;

    fn executor<'a>(
        args: Self::Arguments<'a>,
        context: &Context,
    ) -> Result<Self::Output, EvaluateNodeError>;

    #[doc(hidden)]
    fn description() -> TranslatedString {
        Self::Arguments::description()
    }

    #[doc(hidden)]
    fn call(
        arguments: &ArgumentMapping,
        context: &Context,
    ) -> Result<Node, EvaluateNodeError> {
        Ok(
            Self::executor(Self::Arguments::from_mapping(arguments)?, context)?
                .into_node(),
        )
    }

    #[doc(hidden)]
    fn signature() -> FunctionSignature {
        Self::Arguments::signature()
            .add_return_type(Self::Output::static_node_type())
    }
}

pub trait ConstantEndpoint {
    type Output: IntoNode + GetStaticNodeType;

    fn name() -> &'static str;

    fn description() -> TranslatedString;

    fn executor(context: &Context) -> Self::Output;

    #[doc(hidden)]
    fn call(context: &Context) -> Node {
        let node = Self::executor(context);
        node.into_node()
    }

    #[doc(hidden)]
    fn node_type() -> NodeType {
        Self::Output::static_node_type()
    }
}
