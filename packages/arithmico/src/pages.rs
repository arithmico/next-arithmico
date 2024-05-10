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
mod settings {
    pub mod settings;
}

pub use about::about::*;
pub use calculator::calculator::*;
pub use help::help::*;
pub use settings::settings::*;
