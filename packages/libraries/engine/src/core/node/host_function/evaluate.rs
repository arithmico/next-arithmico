use evaluator::Error;
use node::{HostFunction, Node};

use crate::core::{Context, EvaluateNode};

impl EvaluateNode for node::HostFunction {
    fn evaluate(&self, _context: &Context) -> Result<Node, Error> {
        Ok(HostFunction::new(&self.name))
    }
}
