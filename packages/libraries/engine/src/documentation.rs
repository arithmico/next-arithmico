use std::collections::HashMap;

use ast::{FunctionCall, Symbol};
use common::{
    DecimalFormat, DecimalPlaces, HostApi, HostEndpoint, Language,
    TranslatedString,
};
use serializer::{SerializeNodeOptions, serialize_node};

pub struct DocumentationItem {
    synopsis: TranslatedString,
    description: TranslatedString,
}

impl DocumentationItem {
    pub fn new() -> Self {
        Self {
            synopsis: HashMap::new(),
            description: HashMap::new(),
        }
    }

    pub fn synopsis(&self, language: &Language) -> Option<&String> {
        self.synopsis.get(language)
    }

    pub fn description(&self, language: &Language) -> Option<&String> {
        self.description.get(language)
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
                        &SerializeNodeOptions::new(
                            DecimalPlaces::default(),
                            DecimalFormat::Dot,
                        ),
                    )
                    .expect("serialized"),
                );
                item.synopsis.insert(
                    Language::German,
                    serialize_node(
                        &synopsis_expression,
                        &SerializeNodeOptions::new(
                            DecimalPlaces::default(),
                            DecimalFormat::Comma,
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
                        &SerializeNodeOptions::new(
                            DecimalPlaces::default(),
                            DecimalFormat::Dot,
                        ),
                    )
                    .expect("serialized"),
                );
                item.synopsis.insert(
                    Language::German,
                    serialize_node(
                        &synopsis_expression,
                        &SerializeNodeOptions::new(
                            DecimalPlaces::default(),
                            DecimalFormat::Comma,
                        ),
                    )
                    .expect("serialized"),
                );
                item
            }
        }
    }
}

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
