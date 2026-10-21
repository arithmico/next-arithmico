use editor::{
    editor::{Editor, EditorStateMutation},
    node::{MarkNode, TextNode},
};
use editor_core::{
    EditorContainerNode, EditorLeafNode,
    selection::{SelectionRange, SelectionRangePoint},
};
use leptos::{
    logging::{error, warn},
    prelude::*,
};
use translate::use_translate;
use web_state::use_state;

use crate::state::{EvaluateAction, State, UpdateInputEditorAction};

#[component]
pub fn CaluclatorInput() -> impl IntoView {
    view! { <CalculatorInputEditor /> }
}

#[component]
fn CalculatorInputEditor() -> impl IntoView {
    let state = use_state::<State>();
    let error_spans = state.query_memo(|state| state.error_spans.clone());

    Effect::new(move |_| {
        let error_spans = error_spans.get();

        let s = state.0.read_untracked();
        let editor_state = &s.input_editor_state;

        let Some(content) =
            editor_state.serialize_node(editor_state.get_root_id())
        else {
            warn!("No node to serialize.");
            return;
        };
        if content.is_empty() {
            return;
        }
        let mut segments = Vec::<(String, bool)>::new();
        let mut pos = 0;
        for span in error_spans.iter() {
            let start = span.from_byte_offset();
            let end = span.to_byte_offset();

            if start < pos || start >= content.len() {
                error!("Start offset out of bounds!");
                return;
            }

            if pos < start {
                segments.push((content[pos..start].to_string(), false));
                pos = start;
            }

            let end_char_len = content[end..]
                .chars()
                .next()
                .map(|c| c.len_utf8())
                .unwrap_or(1);
            let end_boundary = (end + end_char_len).min(content.len());

            segments.push((content[pos..end_boundary].to_string(), true));
            pos = end_boundary;
        }

        if pos < content.len() {
            segments.push((content[pos..].to_string(), false));
        }

        drop(s);

        state.dispatch(UpdateInputEditorAction::new(EditorStateMutation::new(
            move |state| {
                let selection_pos =
                    state.get_selection().and_then(|selection| {
                        let anchor_pos = state
                            .get_absolute_offset(
                                selection.get_anchor().get_node_id(),
                                selection.get_anchor().get_offset(),
                            )
                            .ok()?;
                        let focus_pos = state
                            .get_absolute_offset(
                                selection.get_focus().get_node_id(),
                                selection.get_focus().get_offset(),
                            )
                            .ok()?;
                        Some((anchor_pos, focus_pos))
                    });

                state.clear_root_node()?;
                state.clear_selection();
                for (segment, is_highlighted) in segments.clone() {
                    if is_highlighted {
                        let mark_node_id = state.insert_node(
                            MarkNode.into_editor_node(),
                            None,
                            None,
                        )?;
                        state.insert_node(
                            TextNode::new_with_content(segment)
                                .into_editor_node(),
                            Some(mark_node_id),
                            None,
                        )?;
                    } else {
                        state.insert_node(
                            TextNode::new_with_content(segment)
                                .into_editor_node(),
                            None,
                            None,
                        )?;
                    }
                }
                if let Some((anchor_pos, focus_pos)) = selection_pos {
                    let (anchor_node_id, anchor_offset) = state
                        .get_node_id_and_offset_from_absolute_offset(
                            anchor_pos,
                        )?;

                    let (focus_node_id, focus_offset) = state
                        .get_node_id_and_offset_from_absolute_offset(
                            focus_pos,
                        )?;

                    state.set_selection(SelectionRange::new(
                        SelectionRangePoint::new(anchor_node_id, anchor_offset),
                        SelectionRangePoint::new(focus_node_id, focus_offset),
                    ));
                } else {
                    state.clear_selection();
                }
                state.apply_transforms()?;
                state.update_dom()?;
                state.write_selection_to_dom()?;

                Ok(())
            },
        )));
    });

    let update_editor_state: Callback<EditorStateMutation> =
        Callback::new(move |mutation| {
            state.dispatch(UpdateInputEditorAction::new(mutation));
        });

    let translate = use_translate();

    let revision = state.query(|state| state.input_editor_state.get_revision());

    view! {
        <Editor
            id="calculator-input"
            autofocus=true
            data_test_id="calculator-input"
            attr:aria-label=move || {
                translate("calculator.input.label", None).unwrap_or_default()
            }
            attr:data-revision=revision
            on:keydown=move |event| {
                if event.key() == "Enter" {
                    event.prevent_default();
                    let s = state.0.read();
                    let editor_state = &s.input_editor_state;
                    let content: Option<String> = editor_state
                        .serialize_node(editor_state.get_root_id());
                    drop(s);
                    if let Some(content) = content {
                        state.dispatch(EvaluateAction::new(content));
                    }
                }
            }
            class="calculator-input"
            update_editor_state=update_editor_state
        />
    }
}
