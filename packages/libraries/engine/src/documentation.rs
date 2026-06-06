use std::collections::HashMap;

use translate_core::TranslationError;

use crate::{
    Argument, Cardinality,
    core::{
        Context, DecimalFormat, DecimalPlaces, FunctionCall, HostApi,
        HostEndpoint, Language, NodeType, Stack, Symbol, TranslatedString,
        serialize_node,
    },
};

#[derive(Debug, Clone)]
pub struct ParameterDocumentationItem {
    name: String,
    parameter_types: Vec<NodeType>,
    requirement: Cardinality,
    description: TranslatedString,
}

impl ParameterDocumentationItem {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            parameter_types: Vec::new(),
            requirement: Cardinality::Optional,
            description: TranslatedString::new(),
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
        language: &Language,
    ) -> Result<String, TranslationError> {
        self.description
            .get(language)
            .cloned()
            .ok_or(TranslationError::MissingKey(language.to_string()))
    }
}

impl From<&Argument> for ParameterDocumentationItem {
    fn from(argument: &Argument) -> Self {
        let mut item = ParameterDocumentationItem::new();
        item.name = argument.get_name();
        item.parameter_types = argument
            .get_options()
            .node_types()
            .iter()
            .cloned()
            .collect();
        item.requirement = argument.get_options().cardinality();
        item.description = argument.get_description().try_into().unwrap();

        item
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentationItemType {
    Function,
    Constant,
}

#[derive(Clone, Debug)]
pub struct DocumentationItem {
    documentation_type: DocumentationItemType,
    synopsis: TranslatedString,
    description: TranslatedString,
    parameters: Vec<ParameterDocumentationItem>,
    return_types: Vec<NodeType>,
}

impl DocumentationItem {
    pub fn new_function() -> Self {
        Self {
            documentation_type: DocumentationItemType::Function,
            synopsis: HashMap::new(),
            description: HashMap::new(),
            parameters: Vec::new(),
            return_types: Vec::new(),
        }
    }

    pub fn new_constant() -> Self {
        Self {
            documentation_type: DocumentationItemType::Constant,
            synopsis: HashMap::new(),
            description: HashMap::new(),
            parameters: Vec::new(),
            return_types: Vec::new(),
        }
    }

    pub fn get_documentation_type(&self) -> &DocumentationItemType {
        &self.documentation_type
    }

    pub fn get_synopsis(&self, language: &Language) -> Option<&String> {
        self.synopsis.get(language)
    }

    pub fn get_description(&self, language: &Language) -> Option<&String> {
        self.description.get(language)
    }

    pub fn get_parameters(&self) -> &[ParameterDocumentationItem] {
        &self.parameters
    }

    pub fn get_return_types(&self) -> &[NodeType] {
        &self.return_types
    }

    pub fn from_endpoint(name: &str, endpoint: &HostEndpoint) -> Self {
        match endpoint {
            HostEndpoint::Function {
                signature,
                description,
                ..
            } => {
                let mut item = DocumentationItem::new_function();
                item.description = description.clone();
                item.parameters = signature
                    .arguments()
                    .iter()
                    .map(|argument| ParameterDocumentationItem::from(argument))
                    .collect();
                item.return_types =
                    signature.get_return_type().iter().cloned().collect();
                let synopsis_expression = FunctionCall::new(
                    Symbol::new(name),
                    signature
                        .argument_names()
                        .iter()
                        .map(|argument| Symbol::new(&argument))
                        .collect(),
                );
                item.synopsis.insert(
                    Language::English,
                    serialize_node(
                        &synopsis_expression,
                        &Context::new(
                            Stack::new(),
                            DecimalPlaces::default(),
                            DecimalFormat::Dot,
                            HostApi::empty().into(),
                        ),
                    )
                    .expect("serialized"),
                );
                item.synopsis.insert(
                    Language::German,
                    serialize_node(
                        &synopsis_expression,
                        &Context::new(
                            Stack::new(),
                            DecimalPlaces::default(),
                            DecimalFormat::Comma,
                            HostApi::empty().into(),
                        ),
                    )
                    .expect("serialized"),
                );
                item
            }
            HostEndpoint::Constant { description, .. } => {
                let mut item = DocumentationItem::new_constant();
                item.description = description.clone();
                let synopsis_expression = Symbol::new(name);
                item.synopsis.insert(
                    Language::English,
                    serialize_node(
                        &synopsis_expression,
                        &Context::new(
                            Stack::new(),
                            DecimalPlaces::default(),
                            DecimalFormat::Dot,
                            HostApi::empty().into(),
                        ),
                    )
                    .expect("serialized"),
                );
                item.synopsis.insert(
                    Language::German,
                    serialize_node(
                        &synopsis_expression,
                        &Context::new(
                            Stack::new(),
                            DecimalPlaces::default(),
                            DecimalFormat::Comma,
                            HostApi::empty().into(),
                        ),
                    )
                    .expect("serialized"),
                );
                item
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct DocumentationModule {
    name: TranslatedString,
    items: Vec<DocumentationItem>,
}

impl DocumentationModule {
    pub fn name(&self, language: &Language) -> Option<&String> {
        self.name.get(language)
    }

    pub fn items(&self) -> &[DocumentationItem] {
        &self.items
    }
}

pub struct Documentation {
    modules: Vec<DocumentationModule>,
}

impl Documentation {
    pub fn modules(&self) -> &[DocumentationModule] {
        &self.modules
    }
}

impl From<&HostApi> for Documentation {
    fn from(api: &HostApi) -> Self {
        let mut modules: HashMap<String, DocumentationModule> = HashMap::new();
        for (name, endpoint) in api.endpoints() {
            let module_id = endpoint.get_module_id();
            let module = match modules.get_mut(module_id) {
                Some(module) => module,
                None => {
                    modules.insert(
                        module_id.to_string(),
                        DocumentationModule {
                            name: endpoint.get_module_name().clone(),
                            items: vec![],
                        },
                    );
                    modules.get_mut(module_id).expect("module")
                }
            };

            let item = DocumentationItem::from_endpoint(name, endpoint);
            module.items.push(item);
        }

        Documentation {
            modules: modules.into_values().collect(),
        }
    }
}
