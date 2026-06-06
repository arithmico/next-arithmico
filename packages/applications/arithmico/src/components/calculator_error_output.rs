use std::collections::HashMap;

use engine::{Expectation, ParseNodeErrorSummary, SessionError};
use leptos::prelude::*;
use translate::{FormattedMessage, TranslatableObject, use_translate};

#[component]
pub fn CalculatorErrorOutput(error: SessionError) -> impl IntoView {
    view! {
        <>
            {match error {
                SessionError::ParseNodeError(error) => {
                    let translate = use_translate();
                    let translate_expectation = |
                        expectation: &Expectation,
                    | -> Result<String, _> {
                        match expectation {
                            Expectation::Number => {
                                translate(
                                    "calculator.syntax-error.expectation.number",
                                    None,
                                )
                            }
                            Expectation::Symbol => {
                                translate(
                                    "calculator.syntax-error.expectation.symbol",
                                    None,
                                )
                            }
                            Expectation::Boolean => {
                                translate(
                                    "calculator.syntax-error.expectation.boolean",
                                    None,
                                )
                            }
                            Expectation::Token(token) => Ok(format!(r#""{token}""#)),
                        }
                    };

                    view! {
                        <>
                            <h2>
                                <FormattedMessage id="calculator.syntax-error.title" />
                            </h2>
                            {match error.summary() {
                                ParseNodeErrorSummary::MissingParenthesis { .. } => {
                                    view! {
                                        <p>
                                            <FormattedMessage id="calculator.syntax-error.missing-parenthesis" />
                                        </p>
                                    }
                                        .into_any()
                                }
                                ParseNodeErrorSummary::Expectations {
                                    expectations,
                                    actual,
                                    ..
                                } => {
                                    let actual = actual
                                        .map(|actual| format!(r#""{}""#, actual))
                                        .unwrap_or_else(|| {
                                            translate(
                                                    "calculator.syntax-error.actual.end-of-input",
                                                    None,
                                                )
                                                .unwrap_or_default()
                                        });
                                    let mut keys = HashMap::new();
                                    keys.insert("actual".to_string(), actual);
                                    match expectations.as_slice() {
                                        [] => {

                                            view! { <p>SyntaxError</p> }
                                                .into_any()
                                        }
                                        [first] => {
                                            if let Ok(expectation) = translate_expectation(first) {
                                                keys.insert("expectation".to_string(), expectation);
                                            }
                                            view! {
                                                <p>
                                                    <FormattedMessage
                                                        id="calculator.syntax-error.expectation.one"
                                                        keys=keys.into()
                                                    />
                                                </p>
                                            }
                                                .into_any()
                                        }
                                        [first, second] => {
                                            if let Ok(expectation) = translate_expectation(first) {
                                                keys.insert("expectation_1".to_string(), expectation);
                                            }
                                            if let Ok(expectation) = translate_expectation(second) {
                                                keys.insert("expectation_2".to_string(), expectation);
                                            }
                                            view! {
                                                <p>
                                                    <FormattedMessage
                                                        id="calculator.syntax-error.expectation.two"
                                                        keys=keys.into()
                                                    />
                                                </p>
                                            }
                                                .into_any()
                                        }
                                        [rest @ .., last] => {
                                            if let Ok(expectations) = rest
                                                .iter()
                                                .map(|expectation| translate_expectation(expectation))
                                                .collect::<Result<Vec<_>, _>>()
                                            {
                                                keys.insert(
                                                    "expectations".to_string(),
                                                    expectations.join(", "),
                                                );
                                            }
                                            if let Ok(expectation) = translate_expectation(last) {
                                                keys.insert("last_expectation".to_string(), expectation);
                                            }
                                            view! {
                                                <p>
                                                    <FormattedMessage
                                                        id="calculator.syntax-error.expectation.more"
                                                        keys=keys.into()
                                                    />
                                                </p>
                                            }
                                                .into_any()
                                        }
                                    }
                                }
                            }}
                        </>
                    }
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
