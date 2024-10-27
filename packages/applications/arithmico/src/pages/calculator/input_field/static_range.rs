use js_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{InputEvent, Node};

#[derive(Debug, Clone)]
pub struct StaticRange {
    start: usize,
    end: usize,
    start_container: Node,
    end_container: Node,
}

impl StaticRange {
    pub fn new(
        start: usize,
        end: usize,
        start_container: Node,
        end_container: Node,
    ) -> Self {
        debug_assert!(start <= end);
        Self {
            start,
            end,
            start_container,
            end_container,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn start_container(&self) -> Node {
        self.start_container.clone()
    }

    pub fn end_container(&self) -> Node {
        self.end_container.clone()
    }
}

pub fn get_static_ranges(event: &InputEvent) -> Vec<StaticRange> {
    event
        .get_target_ranges()
        .to_vec()
        .into_iter()
        .map(|range| {
            let start_offset: usize =
                js_sys::Reflect::get(&range, &JsValue::from_str("startOffset"))
                    .expect("start offset")
                    .as_f64()
                    .expect("f63") as usize;
            let end_offset =
                js_sys::Reflect::get(&range, &JsValue::from_str("endOffset"))
                    .expect("end offset")
                    .as_f64()
                    .expect("f63") as usize;
            let start_container: Node = js_sys::Reflect::get(
                &range,
                &JsValue::from_str("startContainer"),
            )
            .expect("startContainer")
            .dyn_into()
            .expect("node");

            let end_container: Node = js_sys::Reflect::get(
                &range,
                &JsValue::from_str("endContainer"),
            )
            .expect("endContainer")
            .dyn_into()
            .expect("node");

            StaticRange::new(
                start_offset,
                end_offset,
                start_container,
                end_container,
            )
        })
        .collect::<Vec<_>>()
}
