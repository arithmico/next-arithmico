use headless_components::listbox::ListboxOptionProps;
use yew::{
    prelude::*,
    virtual_dom::{VChild, VNode, VText},
};

use crate::{
    app_context::{AppAction, AppContext},
    components::{Listbox, ListboxButton, ListboxOptions},
};

use icons::expand::ExpandIcon;

#[derive(PartialEq, Properties)]
pub struct DecimalPlacesSelectorProps {}

#[function_component]
pub fn DecimalPlacesSelector(_props: &DecimalPlacesSelectorProps) -> Html {
    let app_context = use_context::<AppContext>().unwrap();
    let decimal_places = app_context.settings.decimal_places;
    let onchange = {
        let app_context = app_context.clone();
        Callback::from(move |value: u8| {
            app_context.dispatch(AppAction::SetDecimalPlaces(value))
        })
    };

    html! {
        <label class={classes!("flex", "items-center", "text-xl")}>
            {"Nachkommastellen"}
            <Listbox<u8>
                on_change={onchange}
                value={decimal_places}
                class={classes!("ml-auto")}
            >
                <ListboxButton<u8>>
                    {decimal_places.to_string()}
                    <ExpandIcon class={classes!(
                        "w-4",
                        "h-4",
                        "ml-auto",
                        "group-data-[expanded=true]:rotate-180"
                    )} />
                </ListboxButton<u8>>
                <div>
                    <ListboxOptions<u8> class={classes!("grid", "grid-cols-3")}>
                        {for (0u8..15u8).into_iter().map(|value| {
                            VChild::new(
                                ListboxOptionProps {
                                    value,
                                    children: Children::new(vec![VNode::VText(VText { text: value.to_string().into() })]),
                                    class: classes!(
                                        "flex",
                                        "items-center",
                                        "justify-center"
                                    )
                                },
                                None
                            )
                        })}
                    </ListboxOptions<u8>>
                </div>
            </Listbox<u8>>
        </label>
    }
}
