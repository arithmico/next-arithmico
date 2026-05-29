use engine::SessionError;
use leptos::prelude::*;
use translate::{FormattedMessage, TranslatableObject};

#[component]
pub fn CalculatorErrorOutput(error: SessionError) -> impl IntoView {
    view! {
        <>
            {match error {
                SessionError::ParseNodeError(error) => {
                    view! {
                        <>
                            <p>{format!("SyntaxError: {}", "")}</p>
                            <p>
                                {format!("{:#?}", &error)
                                    .chars()
                                    .filter(|c| *c == '\n')
                                    .count()}
                            </p>
                            <p>{format!("{:#?}", &error)}</p>
                        </>
                    }
                        .into_any()
                }
                SessionError::SerializeNodeError(_) => {
                    view! { <>SerializationError</> }.into_any()
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
