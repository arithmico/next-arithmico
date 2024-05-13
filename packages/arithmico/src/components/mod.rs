mod app_context {
    pub mod app_context;
}
mod navbar {
    pub mod navbar;
}
mod page {
    pub mod page;
}
mod page_with_navbar {
    pub mod page_with_navbar;
}

pub use app_context::app_context::*;
pub use navbar::navbar::*;
pub use page::page::*;
pub use page_with_navbar::page_with_navbar::*;
