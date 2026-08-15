use node::{DataFrame, IntoNode};

use crate::{Error, EvaluateNode, Options};

impl EvaluateNode for DataFrame {
    fn evaluate<'a>(
        &'a self,
        _options: Options<'a>,
    ) -> Result<node::Node, Error> {
        Ok(self.clone().into_node())
    }
}
