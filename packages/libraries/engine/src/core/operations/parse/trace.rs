use std::cell::RefCell;

use trace::{Span, TracableMut};

use node::Node;

use super::ParseResult;

thread_local! {
    static INPUT_LENGTH: RefCell<Option<usize>> = RefCell::new(None);
}

fn set_input_length(length: usize) {
    INPUT_LENGTH.with_borrow_mut(|v| v.replace(length));
}

fn get_input_length() -> Option<usize> {
    INPUT_LENGTH.with_borrow(|v| v.clone())
}

fn clear_input_length() {
    INPUT_LENGTH.set(None);
}

pub fn with_input_len<I>(f: I) -> impl Fn(&str) -> ParseResult
where
    I: Fn(&str) -> ParseResult + 'static,
{
    move |input| {
        if get_input_length().is_none() {
            set_input_length(input.len());
            let result = f(input);
            clear_input_length();
            result
        } else {
            f(input)
        }
    }
}

pub trait TraceUtils {
    fn with_span_from_parser(self, input: &str, remaining_input: &str) -> Self;

    fn with_trimmed_span_from_parser(
        self,
        input: &str,
        remaining_input: &str,
    ) -> Self;
}

impl TraceUtils for Node {
    fn with_span_from_parser(
        mut self,
        input: &str,
        remaining_input: &str,
    ) -> Self {
        let input_length = get_input_length().expect("input length");
        let span = Span::new(
            input_length - input.len(),
            input_length - remaining_input.len() - 1,
        );
        self.trace_mut().push_span(span);
        self
    }

    fn with_trimmed_span_from_parser(
        mut self,
        input: &str,
        remaining_input: &str,
    ) -> Self {
        let input_length = get_input_length().expect("input length");
        let whitespaces = input.len() - input.trim_start().len();

        let span = Span::new(
            input_length - input.len() + whitespaces,
            input_length - remaining_input.len() - 1,
        );
        self.trace_mut().push_span(span);
        self
    }
}
