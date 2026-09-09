use node::{FunctionSignature, GetStaticNodeType, Node, NodeType};
use translate_core::RenderedTranslatedMessage;

use crate::{
    ArgumentMapping, EndpointMetadata, Error, FunctionArguments, Options,
};

pub type FunctionExecutor =
    fn(&ArgumentMapping, Options) -> Result<Node, Error>;

pub type ConstantExecutor = fn(Options) -> Node;

#[derive(Debug)]
pub enum Endpoint {
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

impl Endpoint {
    pub fn function<F: FunctionEndpoint>(
        module_id: &str,
        module_name: RenderedTranslatedMessage,
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
        module_name: RenderedTranslatedMessage,
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
            Endpoint::Function { metadata, .. } => metadata,
            Endpoint::Constant { metadata, .. } => metadata,
        }
    }

    pub fn endpoint_name(&self) -> &str {
        &self.metadata().endpoint_name
    }

    pub fn module_id(&self) -> &str {
        &self.metadata().module_id
    }

    pub fn module_name(&self) -> &RenderedTranslatedMessage {
        &self.metadata().module_name
    }
}

pub trait FunctionEndpoint {
    type Output: GetStaticNodeType + Into<Node>;
    type Arguments<'a>: FunctionArguments<'a>;

    fn executor<'a>(
        args: Self::Arguments<'a>,
        context: Options,
    ) -> Result<Self::Output, Error>;

    #[doc(hidden)]
    fn name() -> &'static str {
        Self::Arguments::function_name()
    }

    #[doc(hidden)]
    fn description() -> RenderedTranslatedMessage {
        Self::Arguments::function_description()
    }

    #[doc(hidden)]
    fn call(
        arguments: &ArgumentMapping,
        context: Options,
    ) -> Result<Node, Error> {
        Self::executor(Self::Arguments::from_mapping(arguments)?, context)
            .map(Into::into)
    }

    #[doc(hidden)]
    fn signature() -> FunctionSignature {
        Self::Arguments::signature()
            .add_return_type(Self::Output::static_node_type())
    }
}

pub trait ConstantMetadata {
    fn constant_name() -> &'static str;
    fn constant_description() -> RenderedTranslatedMessage;
}

pub trait ConstantEndpoint: ConstantMetadata {
    type Output: Into<Node> + GetStaticNodeType;

    fn executor(context: Options) -> Self::Output;

    #[doc(hidden)]
    fn name() -> &'static str {
        Self::constant_name()
    }

    #[doc(hidden)]
    fn description() -> RenderedTranslatedMessage {
        Self::constant_description()
    }

    #[doc(hidden)]
    fn call(context: Options) -> Node {
        Self::executor(context).into()
    }

    #[doc(hidden)]
    fn node_type() -> NodeType {
        Self::Output::static_node_type()
    }
}
