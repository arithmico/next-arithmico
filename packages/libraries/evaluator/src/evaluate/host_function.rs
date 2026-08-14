use node::{HostFunction, Node};

use crate::{Error, EvaluateNode, Options};

impl EvaluateNode for node::HostFunction {
    fn evaluate(&self, _options: Options) -> Result<Node, Error> {
        Ok(HostFunction::new(&self.name))
    }
}
