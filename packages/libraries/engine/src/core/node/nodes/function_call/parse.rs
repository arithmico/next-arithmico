use pest::iterators::Pair;

use crate::{
    core::node::{Node, NodeError, Rule},
    utils::parse_utils::next_pair_of_rule,
};

use super::FunctionCall;

impl TryFrom<Pair<'_, Rule>> for FunctionCall {
    type Error = NodeError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::function_call => {
                let mut inner_pairs = pair.into_inner();
                let target = Node::try_from(
                    next_pair_of_rule(
                        &mut inner_pairs,
                        Rule::function_call_target,
                    )
                    .into_inner()
                    .next()
                    .unwrap(),
                )?;
                let arguments = next_pair_of_rule(
                    &mut inner_pairs,
                    Rule::function_call_arguments,
                )
                .into_inner()
                .map(|argument| Node::try_from(argument))
                .collect::<Result<Vec<_>, _>>()?;
                Ok(FunctionCall::new(target, arguments))
            }
            _ => Err(NodeError::ParsingError(String::from(
                "failed to convert parse tree to syntax tree",
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread, time::Duration};

    use crate::core::node::*;

    #[test]
    fn parse_function_call() {
        let result = Node::parse("f(1, 2)").unwrap();
        assert_eq!(
            result,
            FunctionCall::new(
                Symbol::new("f"),
                vec![Number::new(1.0).into(), Number::new(2.0).into()]
            )
            .into()
        );
    }

    fn panic_after<T, F>(d: Duration, f: F) -> T
    where
        T: Send + 'static,
        F: FnOnce() -> T,
        F: Send + 'static,
    {
        let (done_tx, done_rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            let val = f();
            done_tx.send(()).expect("Unable to send completion signal");
            val
        });

        match done_rx.recv_timeout(d) {
            Ok(_) => handle.join().expect("Thread panicked"),
            Err(_) => panic!("Thread took too long"),
        }
    }

    #[test]
    fn parse_nested_function_call() {
        let result =
            panic_after(Duration::from_secs(5), || Node::parse("f(f(f(x)))"))
                .unwrap();

        assert_eq!(
            result,
            FunctionCall::new(
                Symbol::new("f"),
                vec![FunctionCall::new(
                    Symbol::new("f"),
                    vec![FunctionCall::new(
                        Symbol::new("f"),
                        vec![Symbol::new("x").into()]
                    )
                    .into()]
                )
                .into()]
            )
            .into()
        );
    }
}
