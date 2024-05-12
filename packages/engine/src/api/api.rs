use std::f64::consts::PI;

use crate::{context::HostApi, node::Node};

pub fn load_host_api() -> HostApi {
    HostApi::builder()
        .add_constant_endpoint("pi".into(), |_context| Node::Number {
            value: PI,
        })
        .build()
}
