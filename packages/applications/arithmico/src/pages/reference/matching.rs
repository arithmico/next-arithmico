use common::Language;
use engine::DocumentationModule;
use frizbee::{Matcher, Pattern};
use leptos::reactive::{computed::Memo, traits::Get, wrappers::read::Signal};

#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceMatch {
    pub description: String,
    pub synopsis: String,
    pub url: String,
    pub score: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceModuleMatch {
    pub max_score: u16,
    pub name: String,
    pub matches: Vec<ReferenceMatch>,
}

pub fn match_reference_modules(
    modules: Signal<Vec<DocumentationModule>>,
    language: Signal<Language>,
    search_query: Signal<String>,
) -> Memo<Vec<ReferenceModuleMatch>> {
    Memo::new(move |_| {
        let modules = modules.get();
        let language = language.get();
        let search_query = search_query.get();
        let mut matcher = Matcher::from_patterns(
            &[Pattern::new(&search_query, Default::default())],
            &Default::default(),
        );

        let mut matches = modules
            .into_iter()
            .filter_map(|module| {
                let mut items = module
                    .items()
                    .iter()
                    .filter_map(|item| {
                        let description =
                            item.get_description(language).unwrap_or_default();
                        let synopsis =
                            item.get_synopsis(language).unwrap_or_default();
                        let description_score =
                            matcher.match_one(&description, 0);
                        let synopsis_score = matcher.match_one(&synopsis, 0);
                        let score = match (synopsis_score, description_score) {
                            (Some(a), Some(b)) => a.score.max(b.score),
                            (Some(a), None) => a.score,
                            (None, Some(b)) => b.score,
                            _ => {
                                return None;
                            }
                        };
                        Some(ReferenceMatch {
                            description,
                            synopsis,
                            score,
                            url: format!(
                                "/reference/{}",
                                item.get_endpoint_name()
                            ),
                        })
                    })
                    .collect::<Vec<_>>();

                items.sort_by(|a, b| b.score.cmp(&a.score));

                let max_score: u16 =
                    items.iter().fold(None, |acc: Option<u16>, val| {
                        if let Some(acc) = acc {
                            Some(acc.max(val.score))
                        } else {
                            Some(val.score)
                        }
                    })?;
                Some(ReferenceModuleMatch {
                    name: module.name(language).unwrap_or_default(),
                    max_score,
                    matches: items,
                })
            })
            .collect::<Vec<_>>();

        matches.sort_by(|a, b| b.max_score.cmp(&a.max_score));

        matches
    })
}
