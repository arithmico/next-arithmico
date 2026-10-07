use editor_core::{EditorCommand, EditorState};

#[derive(Debug, Clone, Copy)]
pub struct DeleteByCutCommand;

impl EditorCommand for DeleteByCutCommand {
    fn apply(&self, state: &mut EditorState) -> Result<(), editor_core::Error> {
        let selection = state.get_selection_or_err()?;

        // cut nothing
        if selection.is_collapsed() {
            return Ok(());
        }

        state.delete_selection_and_preserve_focus_node()?;

        Ok(())
    }
}
