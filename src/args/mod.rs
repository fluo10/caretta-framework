pub mod option;

#[cfg(any(feature = "desktop-gui", feature = "desktop-server"))]
pub mod parser;

pub mod subcommand;