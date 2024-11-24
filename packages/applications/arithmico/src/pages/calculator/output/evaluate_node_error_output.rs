use common::EvaluateNodeError;
use leptos::*;
use translate::{FormattedMessage, TranslatableObject};

#[component]
#[allow(unused_variables)]
pub fn EvaluateNodeErrorOutput(value: EvaluateNodeError) -> impl IntoView {
    view! {
        <dl class="grid gap-4 grid-cols-[auto_1fr]">
            <dt class="font-bold text-right">
                <FormattedMessage id="calculator.error.error_kind" />
            </dt>
            <dd>
                <TranslatableObject value=value.get_error_kind() />
            </dd>
            <dt class="font-bold text-right">
                <FormattedMessage id="calculator.error.description" />
            </dt>
            <dd>
                <TranslatableObject value=value.get_message().clone() />
            </dd>
        </dl>
    }
}
