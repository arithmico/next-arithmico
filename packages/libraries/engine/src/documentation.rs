use std::collections::HashMap;

use translate_core::{TranslationError, TranslationTemplate};

use crate::core::{
    Context, DecimalFormat, DecimalPlaces, FunctionCall, HostApi, HostEndpoint, Language, NodeType, Stack, Symbol, TranslatedString, serialize_node
};

#[derive(Debug, Clone)]
pub struct ParameterDocumentItem {
    name: String,
    description: TranslationTemplate,
}

impl ParameterDocumentItem {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: TranslationTemplate::new(),
        }
    }

    pub fn with(name: &String, description: &TranslationTemplate) -> Self {
        let mut item = ParameterDocumentItem::new();
        item.name = name.clone();
        item.description = description.clone();

        item
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self, language: &Language) -> Result<String, TranslationError> {
        self.description.translate(language.clone())
    }
}

#[derive(Clone, Debug)]
pub struct DocumentationItem {
    synopsis: TranslatedString,
    description: TranslatedString,
    parameters: Vec<ParameterDocumentItem>,
    return_type: NodeType,
}

impl DocumentationItem {
    pub fn new() -> Self {
        Self {
            synopsis: HashMap::new(),
            description: HashMap::new(),
            parameters: Vec::new(),
            return_type: NodeType::Any,
        }
    }

    pub fn synopsis(&self, language: &Language) -> Option<&String> {
        self.synopsis.get(language)
    }

    pub fn description(&self, language: &Language) -> Option<&String> {
        self.description.get(language)
    }

    pub fn parameters(&self) -> &Vec<ParameterDocumentItem> {
        &self.parameters
    }

    pub fn return_type(&self) -> &NodeType {
        &self.return_type
    }

    pub fn from_endpoint(name: &str, endpoint: &HostEndpoint) -> Self {
        match endpoint {
            HostEndpoint::Function {
                signature,
                description,
                ..
            } => {
                let mut item = DocumentationItem::new();
                item.description = description.clone();
                item.parameters = signature
                    .arguments()
                    .iter()
                    .map(|argument| ParameterDocumentItem::with(&argument.name(), &argument.description()))
                    .collect();
                item.return_type = signature.get_return_type().unwrap().clone();
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
                let mut item = DocumentationItem::new();
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
