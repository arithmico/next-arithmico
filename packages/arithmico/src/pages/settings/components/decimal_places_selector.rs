use headless_components::listbox::ListboxOptionProps;
use yew::{
    prelude::*,
    virtual_dom::{VChild, VNode, VText},
};

use crate::components::{Listbox, ListboxButton, ListboxOptions};

use icons::expand::ExpandIcon;

#[derive(PartialEq, Properties)]
pub struct DecimalPlacesSelectorProps {}

#[function_component]
pub fn DecimalPlacesSelector(_props: &DecimalPlacesSelectorProps) -> Html {
    let decimal_places = use_state(|| 3);
    let onchange = {
        let decimal_places = decimal_places.clone();
        Callback::from(move |value: u8| decimal_places.set(value))
    };

    html! {
        <label class={classes!("flex", "items-center", "text-xl")}>
            {"Nachkommastellen"}
            <Listbox<u8>
                on_change={onchange}
                value={(*decimal_places).clone()}
                class={classes!("ml-auto")}
            >
                <ListboxButton<u8>>
                    {(*decimal_places).to_string()}
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
