use std::collections::HashMap;

use common::Language;
use evaluator::{Api, Endpoint};
use node::{Argument, Cardinality, FunctionCall, Node, NodeType, Symbol};
use translate_core::{
    RenderedTranslatedMessage, Translatable, TranslationError,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParameterDocumentationItem {
    name: String,
    parameter_types: Vec<NodeType>,
    requirement: Cardinality,
    description: RenderedTranslatedMessage,
}

impl ParameterDocumentationItem {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            parameter_types: Vec::new(),
            requirement: Cardinality::Optional,
            description: RenderedTranslatedMessage::default(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_parameter_types(&self) -> &[NodeType] {
        &self.parameter_types
    }

    pub fn get_requirement(&self) -> &Cardinality {
        &self.requirement
    }

    pub fn get_description(
        &self,
        language: Language,
    ) -> Result<String, TranslationError> {
        self.description.translate(language)
    }
}

impl From<&Argument> for ParameterDocumentationItem {
    fn from(argument: &Argument) -> Self {
        let mut item = ParameterDocumentationItem::new();
        item.name = argument.get_name().to_string();
        item.parameter_types = argument
            .get_options()
            .node_types()
            .iter()
            .cloned()
            .collect();
        item.requirement = argument.get_options().cardinality();
        item.description = argument.get_description();

        item
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub enum DocumentationItemType {
    Function,
    Constant,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DocumentationItem {
    endpoint_name: String,
    documentation_type: DocumentationItemType,
    synopsis: Node,
    description: RenderedTranslatedMessage,
    parameters: Vec<ParameterDocumentationItem>,
    return_types: Vec<NodeType>,
}

impl DocumentationItem {
    pub fn get_documentation_type(&self) -> &DocumentationItemType {
        &self.documentation_type
    }

    pub fn get_synopsis(&self) -> &Node {
        &self.synopsis
    }

    pub fn get_description(
        &self,
        language: Language,
    ) -> Result<String, TranslationError> {
        self.description.translate(language)
    }

    pub fn get_parameters(&self) -> &[ParameterDocumentationItem] {
        &self.parameters
    }

    pub fn get_return_types(&self) -> &[NodeType] {
        &self.return_types
    }

    pub fn get_endpoint_name(&self) -> &str {
        &self.endpoint_name
    }

    pub fn from_endpoint(name: &str, endpoint: &Endpoint) -> Self {
        match endpoint {
            Endpoint::Function {
                metadata,
                signature,
                ..
            } => {
                let description = metadata.description().clone();
                let parameters = signature
                    .arguments()
                    .iter()
                    .map(ParameterDocumentationItem::from)
                    .collect::<Vec<_>>();
                let return_types = signature
                    .get_return_type()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                let synopsis = FunctionCall::new(
                    Symbol::new(metadata.endpoint_name()),
                    signature
                        .argument_names()
                        .iter()
                        .map(|argument| Symbol::new(argument))
                        .collect(),
                );

                Self {
                    endpoint_name: name.to_string(),
                    documentation_type: DocumentationItemType::Function,
                    synopsis,
                    description,
                    parameters,
                    return_types,
                }
            }
            Endpoint::Constant { metadata, .. } => {
                let description = metadata.description().clone();
                let synopsis = Symbol::new(name);
                Self {
                    endpoint_name: name.to_string(),
                    documentation_type: DocumentationItemType::Constant,
                    synopsis,
                    description,
                    parameters: vec![],
                    // TODO: get constant type
                    return_types: vec![],
                }
            }
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DocumentationModule {
    name: RenderedTranslatedMessage,
    items: Vec<DocumentationItem>,
}

impl DocumentationModule {
    // TODO: return result
    pub fn name(&self, language: Language) -> Option<String> {
        self.name.translate(language).ok()
    }

    pub fn items(&self) -> &[DocumentationItem] {
        &self.items
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Documentation {
    pub modules: Vec<DocumentationModule>,
}

impl Documentation {
    pub fn modules(&self) -> &[DocumentationModule] {
        &self.modules
    }

    pub fn find_endpoint(
        &self,
        endpoint_name: &str,
    ) -> Option<DocumentationItem> {
        self.modules()
            .iter()
            .flat_map(|module| module.items())
            .find(|item| item.endpoint_name == endpoint_name)
            .cloned()
    }
}

impl From<&Api> for Documentation {
    fn from(api: &Api) -> Self {
        let mut modules: HashMap<String, DocumentationModule> = HashMap::new();
        for (name, endpoint) in api.endpoints() {
            let module_id = endpoint.module_id();
            let module = match modules.get_mut(module_id) {
                Some(module) => module,
                None => modules.entry(module_id.to_string()).or_insert(
                    DocumentationModule {
                        name: endpoint.module_name().clone(),
                        items: vec![],
                    },
                ),
            };

            let item = DocumentationItem::from_endpoint(name, endpoint);
            module.items.push(item);
        }

        Documentation {
            modules: modules.into_values().collect(),
        }
    }
}
