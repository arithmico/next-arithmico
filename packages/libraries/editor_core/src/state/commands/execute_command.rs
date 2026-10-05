use crate::{EditorCommand, state::EditorState};

impl EditorState {
    pub fn execute_command(
        &mut self,
        command: Box<dyn EditorCommand>,
    ) -> Result<(), crate::Error> {
        command.apply(self);
        self.apply_transforms();
        self.update_dom()?;
        self.write_selection_to_dom()
    }
}
