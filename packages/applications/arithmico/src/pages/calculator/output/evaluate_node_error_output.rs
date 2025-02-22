use common::EvaluateNodeError;
use leptos::prelude::*;
use translate::{FormattedMessage, TranslatableObject};

#[component]
#[allow(unused_variables)]
pub fn EvaluateNodeErrorOutput(value: EvaluateNodeError) -> impl IntoView {
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
                    <TranslatableObject value=value.get_error_kind() />
                </dd>
                <dt>
                    <FormattedMessage id="calculator.error.description" />
                </dt>
                <dd>
                    <TranslatableObject value=value.get_message().clone() />
                </dd>
            </dl>
        </>
    }
}
