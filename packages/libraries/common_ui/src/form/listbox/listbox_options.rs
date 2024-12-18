use leptos::prelude::*;
use leptos_dom::Transparent;

#[component(transparent)]
pub fn ListboxOptions(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let options = children
        .map(|children| {
            children()
                .as_children()
                .iter()
                .filter_map(|child| child.as_transparent())
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    ListboxOptionsDefinition { class, options }
}

#[derive(Clone)]
pub(super) struct ListboxOptionsDefinition {
    pub class: Option<String>,
    pub options: Vec<Transparent>,
}

impl IntoView for ListboxOptionsDefinition {
    fn into_view(self) -> View {
        Transparent::new(self).into_view()
    }
}

impl From<Option<&View>> for ListboxOptionsDefinition {
    fn from(value: Option<&View>) -> Self {
        value
            .expect("options")
            .as_transparent()
            .expect("transparent")
            .downcast_ref::<ListboxOptionsDefinition>()
            .expect("ListboxOptionsDefinition")
            .clone()
    }
}
