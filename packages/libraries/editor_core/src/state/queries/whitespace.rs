pub mod get_all_white_spaces;
pub mod get_all_whitespaces_ending_before;
pub mod get_whitespace;

#[derive(Debug, Clone)]
pub struct EditorWhitespace {
    pub node_id: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}
