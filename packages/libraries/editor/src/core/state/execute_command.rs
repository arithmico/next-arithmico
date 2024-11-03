use crate::core::EditorCommand;

use super::EditorState;

impl EditorState {
    pub fn execute_command(&mut self, command: Box<dyn EditorCommand>) {
        command.apply(self);
        self.update_dom();
        self.write_selection_to_dom();
    }
}
