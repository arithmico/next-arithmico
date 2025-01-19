use editor::{
    editor::Editor,
    editor_provider::EditorProvider,
    node::{MarkNode, TextNode},
    use_editor_context::use_editor_context,
};
use editor_core::{
    selection::{SelectionRange, SelectionRangePoint},
    EditorContainerNode, EditorLeafNode,
};
use leptos::prelude::*;

use crate::{
    class_names, pages::calculator::use_error_trace::use_error_trace,
    state::AppAction, utils::expect_dispatch,
};

#[component]
pub fn CaluclatorInput() -> impl IntoView {
    view! {
        <EditorProvider>
            <CalculatorInputEditor />
        </EditorProvider>
    }
}

#[component]
fn CalculatorInputEditor() -> impl IntoView {
    let editor_state = use_editor_context();
    let dispatch = expect_dispatch();
    let error_trace = use_error_trace();

    Effect::new(move |_| {
        if let Some(trace) = error_trace.get() {
            let content = editor_state
                .with_untracked(|state| {
                    state.serialize_node(state.get_root_id())
                })
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
            editor_state.update_untracked(move |state| {
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
                for (segment, is_highlighted) in segments {
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
                        .get_node_id_and_offset_from_absolute_offset(anchor_pos)
                        .expect("anchor pos");

                    let (focus_node_id, focus_offset) = state
                        .get_node_id_and_offset_from_absolute_offset(focus_pos)
                        .expect("focus pos");

                    state.set_selection(SelectionRange::new(
                        SelectionRangePoint::new(anchor_node_id, anchor_offset),
                        SelectionRangePoint::new(focus_node_id, focus_offset),
                    ));
                } else {
                    state.clear_selection();
                }
                state.apply_transforms();
                state.update_dom();
                state.write_selection_to_dom();
            });
        }
    });

    view! {
        <Editor
            id="calculator-input"
            autofocus=true
            data_test_id="calculator-input"
            on:keydown=move |event| {
                if event.key() == "Enter" {
                    event.prevent_default();
                    let content: Option<String> = editor_state
                        .with_untracked(|state| {
                            state.serialize_node(state.get_root_id())
                        });
                    if let Some(content) = content {
                        dispatch.run(AppAction::Evaluate(content))
                    }
                }
            }
            class=class_names!(
                "p-2",
                "text-xl",
                "whitespace-pre-wrap",
                "rounded-sm",
                "outline-none",
                "border-2",
                "theme-light:text-black",
                "theme-dark:text-white",
                "theme-light:bg-white",
                "theme-dark:bg-neutral-800",
                "theme-light:border-neutral-300",
                "theme-dark:border-neutral-700",
                "theme-light:focus-visible:border-neutral-700",
                "theme-dark:focus-visible:border-neutral-500"
            )
        />
    }
}
