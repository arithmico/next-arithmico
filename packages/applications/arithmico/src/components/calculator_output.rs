use leptos::prelude::*;

use engine::{Context, Node, Serialize, SessionError};

use crate::components::CalculatorErrorOutput;

#[component]
pub fn CalculatorOutput(
    #[prop(into)] value: Signal<Option<Result<Node, SessionError>>>,
    #[prop(into)] context: Signal<Context>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <output
            for="calculator-output"
            data-testid="calculator-output"
            class=format!(
                "calculator-output {}",
                class.unwrap_or(String::new()),
            )
        >
            {move || {
                let context = context.get();
                match value.get() {
                    Some(result) => {
                        match result {
                            Ok(value) => {
                                view! { <MathNode value=value context=context /> }
                                    .into_any()
                            }
                            Err(error) => {
                                view! { <CalculatorErrorOutput error=error /> }.into_any()
                            }
                        }
                    }
                    None => view! { <>" "</> }.into_any(),
                }
            }}
        </output>
    }
}

#[component]
fn MathNode(value: Node, context: Context) -> impl IntoView {
    value.serialize(&context)
    /*match value {
        Node::Boolean(boolean) => todo!(),
        Node::Sum(sum) => todo!(),
        Node::Negate(negate) => todo!(),
        Node::Product(product) => todo!(),
        Node::Division(division) => todo!(),
        Node::Power(power) => todo!(),
        Node::Tensor(tensor) => todo!(),
        Node::Number(number) => todo!(),
        Node::Symbol(symbol) => todo!(),
        Node::Function(function) => todo!(),
        Node::FunctionCall(function_call) => todo!(),
        Node::And(and) => todo!(),
        Node::Or(or) => todo!(),
        Node::Equals(equals) => todo!(),
        Node::LessThan(less_than) => todo!(),
        Node::LessThanOrEquals(less_than_or_equals) => todo!(),
        Node::GreaterThan(greater_than) => todo!(),
        Node::GreaterThanOrEquals(greater_than_or_equals) => todo!(),
        Node::HostFunction(host_function) => todo!(),
        Node::Definition(definition) => todo!(),
    }*/
}
