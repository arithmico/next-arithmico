use std::collections::HashMap;

use engine::SessionError;
use leptos::prelude::*;
use translate::{FormattedMessage, TranslatableObject, use_translate};

#[component]
pub fn CalculatorErrorOutput(error: SessionError) -> impl IntoView {
    view! {
        <>
            {match error {
                SessionError::ParseNodeError(error) => {
                    let translate = use_translate();

                    view! { <>Error</> }
                        .into_any()
                }
                SessionError::SerializeNodeError(_) => {

                    view! { <>SerializationError</> }
                        .into_any()
                }
                SessionError::EvaluateNodeError(error) => {
                    view! {
                        <>
                            <h2>
                                <FormattedMessage id="calculator.evaluate_node_error" />
                            </h2>
                            <dl>
                                <dt>
                                    <FormattedMessage id="calculator.error.error_kind" />
                                </dt>
                                <dd>
                                    <TranslatableObject value=error.get_error_kind() />
                                </dd>
                                <dt>
                                    <FormattedMessage id="calculator.error.description" />
                                </dt>
                                <dd>
                                    <TranslatableObject value=error.get_message().clone() />
                                </dd>
                            </dl>
                        </>
                    }
                        .into_any()
                }
            }}
        </>
    }
}
