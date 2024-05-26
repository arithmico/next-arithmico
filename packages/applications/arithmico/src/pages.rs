mod about {
    pub mod about;
}
mod calculator {
    pub mod calculator;
    mod components;
}
mod help {
    pub mod help;
}
mod history {
    pub mod history;
}
mod definitions {
    pub mod definitions;
}

mod settings;
pub use about::about::*;
pub use calculator::calculator::*;
pub use definitions::definitions::*;
pub use help::help::*;
pub use history::history::*;
pub use settings::*;
