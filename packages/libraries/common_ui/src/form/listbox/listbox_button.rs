use leptos::prelude::*;
use leptos_dom::Transparent;

#[component(transparent)]
pub fn ListboxButton(
    children: ChildrenFn,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    ListboxButtonDefinition { children, class }
}

#[derive(Clone)]
pub(super) struct ListboxButtonDefinition {
    pub class: Option<String>,
    pub children: ChildrenFn,
}

impl IntoView for ListboxButtonDefinition {
    fn into_view(self) -> View {
        Transparent::new(self).into_view()
    }
}

impl From<Option<&View>> for ListboxButtonDefinition {
    fn from(value: Option<&View>) -> Self {
        value
            .expect("ListboxButton")
            .as_transparent()
            .expect("transparent")
            .downcast_ref::<ListboxButtonDefinition>()
            .expect("ListboxButtonDefinition")
            .clone()
    }
}
