use std::rc::Rc;

use engine::Session;
use yew::{Reducible, UseReducerHandle};

#[derive(Debug, PartialEq, Clone)]
pub struct CalculatorState {
    session: Session,
}

impl CalculatorState {
    pub fn new() -> Self {
        CalculatorState {
            session: Session::new(),
        }
    }

    pub fn session(&self) -> &Session {
        &self.session
    }
}

impl Reducible for CalculatorState {
    type Action = String;

    fn reduce(self: Rc<Self>, input: Self::Action) -> Rc<Self> {
        CalculatorState {
            session: self.session.push(&input),
        }
        .into()
    }
}

pub type CalculatorContext = UseReducerHandle<CalculatorState>;
