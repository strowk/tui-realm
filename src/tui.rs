//! ## tui
//!
//! `tui` just exposes the ratatui modules, in order to include the entire library inside realm

#[cfg(feature = "ratatui")]
pub use ratatui::*;
#[cfg(not(feature = "ratatui"))]
pub use tui::*;
