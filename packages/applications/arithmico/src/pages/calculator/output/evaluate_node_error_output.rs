use common::EvaluateNodeError;
use leptos::prelude::*;
use translate::{FormattedMessage, TranslatableObject};

#[component]
#[allow(unused_variables)]
pub fn EvaluateNodeErrorOutput(value: EvaluateNodeError) -> impl IntoView {
    view! {
        <>
            <h2 class="mb-6 text-xl font-bold">
                <FormattedMessage id="calculator.evaluate_node_error" />
            </h2>
            <dl class="grid gap-4 px-4 text-base grid-cols-[auto_1fr]">
                <dt class="font-semibold">
                    <FormattedMessage id="calculator.error.error_kind" />
                </dt>
                <dd>
                    <TranslatableObject value=value.get_error_kind() />
                </dd>
                <dt class="font-semibold">
                    <FormattedMessage id="calculator.error.description" />
                </dt>
                <dd>
                    <TranslatableObject value=value.get_message().clone() />
                </dd>
            </dl>
        </>
    }
}
