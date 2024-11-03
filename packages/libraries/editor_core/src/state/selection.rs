pub mod get_selection;
pub mod read_from_dom;
pub mod set_selection;
pub mod write_to_dom;

mod selection_range;
mod selection_range_point;

pub use selection_range::SelectionRange;
pub use selection_range_point::SelectionRangePoint;
