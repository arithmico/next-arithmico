use std::rc::Rc;

use engine::{HostApi, Session};
use yew::{Reducible, UseReducerHandle};

#[derive(Debug, PartialEq, Clone)]
pub struct CalculatorState {
    session: Session,
}

impl CalculatorState {
    pub fn new() -> Self {
        let host_api = Rc::new(HostApi::builder().build());
        CalculatorState {
            session: Session::new(host_api),
        }
    }

    pub fn session(&self) -> &Session {
        &self.session
    }
}

impl Reducible for CalculatorState {
    type Action = String;

    fn reduce(self: Rc<Self>, input: Self::Action) -> Rc<Self> {
        if input.is_empty() {
            return self;
        }

        CalculatorState {
            session: self.session.push(&input),
        }
        .into()
    }
}

pub type CalculatorContext = UseReducerHandle<CalculatorState>;
