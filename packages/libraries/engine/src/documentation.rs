use std::collections::HashMap;

use crate::core::{
    Context, DecimalFormat, DecimalPlaces, FunctionCall, HostApi, HostEndpoint,
    Language, Node, NodeType, Stack, Symbol, TranslatedString,
    get_argument_separator, serialize_node,
};

#[derive(Clone, Debug)]
pub struct DocumentationItem {
    synopsis: TranslatedString,
    typed_synopsis: TranslatedString,
    description: TranslatedString,
}

impl DocumentationItem {
    pub fn new() -> Self {
        Self {
            synopsis: HashMap::new(),
            typed_synopsis: HashMap::new(),
            description: HashMap::new(),
        }
    }

    pub fn synopsis(&self, language: &Language) -> Option<&String> {
        self.synopsis.get(language)
    }

    pub fn typed_synopsis(&self, language: &Language) -> Option<&String> {
        self.typed_synopsis.get(language)
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
                let context_english = &Context::new(
                    Stack::new(),
                    DecimalPlaces::default(),
                    DecimalFormat::Dot,
                    HostApi::empty().into(),
                );
                item.synopsis.insert(
                    Language::English,
                    serialize_node(&synopsis_expression, context_english)
                        .expect("serialized"),
                );
                let context_german = &Context::new(
                    Stack::new(),
                    DecimalPlaces::default(),
                    DecimalFormat::Comma,
                    HostApi::empty().into(),
                );
                item.synopsis.insert(
                    Language::German,
                    serialize_node(&synopsis_expression, context_german)
                        .expect("serialized"),
                );

                item.typed_synopsis.insert(Language::English, {
                    let typed_arguments = signature
                        .arguments()
                        .iter()
                        .map(|argument| {
                            let node_types = argument.options().node_types();
                            let node_type = if node_types.len() != 1 {
                                panic!("argument has more than one node type")
                            } else {
                                node_types.iter().next().unwrap().to_string()
                            };

                            format!("{}: {}", argument.name(), node_type)
                        })
                        .collect::<Vec<_>>()
                        .join(&get_argument_separator(context_english));

                    let return_type = signature
                        .return_type()
                        .iter()
                        .next()
                        .unwrap()
                        .to_string();

                    format!("{}({}) -> {}", name, typed_arguments, return_type)
                });
                item.typed_synopsis.insert(Language::German, {
                    let typed_arguments = signature
                        .arguments()
                        .iter()
                        .map(|argument| {
                            let node_types = argument.options().node_types();
                            let node_type = if node_types.len() != 1 {
                                panic!("argument has more than one node type")
                            } else {
                                node_types.iter().next().unwrap().to_string()
                            };

                            format!("{}: {}", argument.name(), node_type)
                        })
                        .collect::<Vec<_>>()
                        .join(&get_argument_separator(context_german));

                    let return_type = signature
                        .return_type()
                        .iter()
                        .next()
                        .unwrap()
                        .to_string();

                    format!("{}({}) -> {}", name, typed_arguments, return_type)
                });

                item
            }
            HostEndpoint::Constant { description, .. } => {
                let mut item = DocumentationItem::new();
                item.description = description.clone();
                let synopsis_expression = Symbol::new(name);
                let context_english = &Context::new(
                    Stack::new(),
                    DecimalPlaces::default(),
                    DecimalFormat::Dot,
                    HostApi::empty().into(),
                );
                item.synopsis.insert(
                    Language::English,
                    serialize_node(&synopsis_expression, context_english)
                        .expect("serialized"),
                );
                let context_german = &Context::new(
                    Stack::new(),
                    DecimalPlaces::default(),
                    DecimalFormat::Comma,
                    HostApi::empty().into(),
                );
                item.synopsis.insert(
                    Language::German,
                    serialize_node(&synopsis_expression, context_german)
                        .expect("serialized"),
                );

                item.typed_synopsis.insert(Language::English, {
                    let node_type = NodeType::Number.to_string();
                    format!("{}: {}", name, node_type)
                });
                item.typed_synopsis.insert(Language::German, {
                    let node_type = NodeType::Number.to_string();
                    format!("{}: {}", name, node_type)
                });

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
