use editor::{
    editor::{Editor, EditorStateMutation},
    node::{MarkNode, TextNode},
};
use editor_core::{
    selection::{SelectionRange, SelectionRangePoint},
    EditorContainerNode, EditorLeafNode,
};
use leptos::prelude::*;
use web_state::WebState;

use crate::state::{EvaluateAction, State, UpdateInputEditorAction};

#[component]
pub fn CaluclatorInput() -> impl IntoView {
    view! { <CalculatorInputEditor /> }
}

#[component]
fn CalculatorInputEditor() -> impl IntoView {
    let state = State::expect_state();
    let error_trace = state.select(|state| state.current_error_trace);

    Effect::new(move |_| {
        if let Some(trace) = error_trace.get() {
            let editor_state = state.get().input_editor_state;
            let content = editor_state
                .serialize_node(editor_state.get_root_id())
                .expect("field content");
            let mut segments = Vec::<(String, bool)>::new();
            let mut pos = 0;
            for span in trace.spans() {
                let start = span.start();
                let end = span.end();
                assert!(start >= pos);
                assert!(start < content.len());
                if pos < start {
                    segments.push((content[pos..start].to_string(), false));
                    pos = start;
                }
                segments.push((content[pos..=end].to_string(), true));
                pos = end + 1;
            }

            if pos < content.len() {
                segments.push((content[pos..].to_string(), false));
            }

            state.dispatch_untracked(&UpdateInputEditorAction::new(
                EditorStateMutation::new(move |state| {
                    let selection_pos = state
                        .get_selection()
                        .map(|selection| {
                            let anchor_pos = state.get_absolute_offset(
                                selection.get_anchor().get_node_id(),
                                selection.get_anchor().get_offset(),
                            )?;
                            let focus_pos = state.get_absolute_offset(
                                selection.get_focus().get_node_id(),
                                selection.get_focus().get_offset(),
                            )?;
                            Some((anchor_pos, focus_pos))
                        })
                        .flatten();

                    state.clear_root_node();
                    state.clear_selection();
                    for (segment, is_highlighted) in segments.clone() {
                        if is_highlighted {
                            let mark_node_id = state.insert_node(
                                MarkNode::new().into_editor_node(),
                                None,
                                None,
                            );
                            state.insert_node(
                                TextNode::new_with_content(segment)
                                    .into_editor_node(),
                                Some(mark_node_id),
                                None,
                            );
                        } else {
                            state.insert_node(
                                TextNode::new_with_content(segment)
                                    .into_editor_node(),
                                None,
                                None,
                            );
                        }
                    }
                    if let Some((anchor_pos, focus_pos)) = selection_pos {
                        let (anchor_node_id, anchor_offset) = state
                            .get_node_id_and_offset_from_absolute_offset(
                                anchor_pos,
                            )
                            .expect("anchor pos");

                        let (focus_node_id, focus_offset) = state
                            .get_node_id_and_offset_from_absolute_offset(
                                focus_pos,
                            )
                            .expect("focus pos");

                        state.set_selection(SelectionRange::new(
                            SelectionRangePoint::new(
                                anchor_node_id,
                                anchor_offset,
                            ),
                            SelectionRangePoint::new(
                                focus_node_id,
                                focus_offset,
                            ),
                        ));
                    } else {
                        state.clear_selection();
                    }
                    state.apply_transforms();
                    state.update_dom();
                    state.write_selection_to_dom();
                }),
            ));
        }
    });

    let update_editor_state: Callback<EditorStateMutation> =
        Callback::new(move |mutation| {
            state.dispatch_untracked(&UpdateInputEditorAction::new(mutation));
        });

    view! {
        <Editor
            id="calculator-input"
            autofocus=true
            data_test_id="calculator-input"
            on:keydown=move |event| {
                if event.key() == "Enter" {
                    event.prevent_default();
                    let editor_state = state.get().input_editor_state;
                    let content: Option<String> = editor_state
                        .serialize_node(editor_state.get_root_id());
                    if let Some(content) = content {
                        state.dispatch(&EvaluateAction::new(content));
                    }
                }
            }
            class="calculator-input"
            update_editor_state=update_editor_state
        />
    }
}
