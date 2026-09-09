use node::{DataFrame, GetNodeType};

use crate::{Error, EvaluateNode, Options};

impl EvaluateNode for DataFrame {
    fn evaluate<'a>(
        &'a self,
        _options: Options<'a>,
    ) -> Result<node::Node, Error> {
        if !cfg!(feature = "datatype_data_frame") {
            return Err(Error::unsupported_datatype(self.node_type()));
        }

        Ok(self.clone().into())
    }
}
