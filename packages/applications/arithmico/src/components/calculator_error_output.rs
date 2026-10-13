use engine_protocol::Error;
use leptos::prelude::*;
use translate::{FormattedMessage, TranslatableObject};

#[component]
pub fn CalculatorErrorOutput(error: Error) -> impl IntoView {
    view! {
        <>
            {match error {
                Error::ParseNodeError(error) => {
                    (move || {
                        let error = error.clone();

                        view! {
                            <>
                                <h2>
                                    <FormattedMessage id="calculator.syntax-error.title" />
                                </h2>
                                <TranslatableObject value=error />
                            </>
                        }
                    })
                        .into_any()
                }
                Error::EvaluateNodeError(error) => {
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
