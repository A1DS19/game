pub mod app;
pub mod window;

pub use log::{debug, error, info, trace, warn};
pub use piston_window;

extern crate pretty_env_logger;

#[macro_use]
extern crate log;

pub fn setup() {
    pretty_env_logger::init();
}
