use crate::EditorState;

use super::{
    SelectionRange, SelectionRangePoint,
    selection_range::AbsoluteSelectionRange,
};

impl EditorState {
    pub fn get_absolute_selection(
        &self,
    ) -> Result<AbsoluteSelectionRange, crate::Error> {
        let selection = self.get_selection_or_err()?;
        let focus = selection.get_focus();
        let abs_focus =
            self.get_absolute_offset(focus.get_node_id(), focus.get_offset())?;
        let anchor = selection.get_anchor();
        let abs_anchor = self
            .get_absolute_offset(anchor.get_node_id(), anchor.get_offset())?;

        Ok(AbsoluteSelectionRange::new(abs_focus, abs_anchor))
    }

    pub fn convert_absolute_selection_to_selection(
        &self,
        selection: AbsoluteSelectionRange,
    ) -> Result<SelectionRange, crate::Error> {
        let (focus_id, focus_offset) = self
            .get_node_id_and_offset_from_absolute_offset(selection.focus())?;
        let (anchor_id, anchor_offset) = self
            .get_node_id_and_offset_from_absolute_offset(selection.anchor())?;

        Ok(SelectionRange::new(
            SelectionRangePoint::new(anchor_id, anchor_offset),
            SelectionRangePoint::new(focus_id, focus_offset),
        ))
    }
}
