use web_sys::InputEvent;

use super::{
    absolute_range::AbsoluteRange, node_utils::target_node,
    static_range::get_static_ranges,
};

#[derive(Debug, Clone)]
pub enum InputAction {
    InsertText { data: String, range: AbsoluteRange },
}

impl InputAction {
    pub fn apply(&self, text: &str) -> Result<(String, AbsoluteRange), String> {
        match self {
            InputAction::InsertText { data, range } => {
                let mut text = text.to_string();
                text.replace_range(range.start()..range.end(), data);
                let cursor_pos = range.start() + data.len();
                Ok((text, AbsoluteRange::new(cursor_pos, cursor_pos)))
            }
        }
    }
}

impl TryFrom<&InputEvent> for InputAction {
    type Error = String;

    fn try_from(event: &InputEvent) -> Result<Self, Self::Error> {
        let input_type = event.input_type();
        let target = target_node(&event);
        match input_type.as_str() {
            "insertText" => {
                let data = event.data().ok_or("missing insert text data")?;
                let ranges = get_static_ranges(&event);
                if ranges.len() != 1 {
                    Err(format!("more than one static range: {}", ranges.len()))
                } else {
                    let range = AbsoluteRange::try_from((
                        &target,
                        ranges.get(0).expect("range"),
                    ))
                    .expect("absolute range");

                    Ok(InputAction::InsertText { data, range })
                }
            }
            _ => Err(format!("unsupported input action: {}", input_type)),
        }
    }
}
