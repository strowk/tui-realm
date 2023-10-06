//! ## Terminal
//!
//! terminal bridge adapter for crossterm

use crate::terminal::{TerminalBridge, TerminalError, TerminalResult};
use crate::tui::backend::CrosstermBackend;
use crate::Terminal;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
#[cfg(target_family = "unix")]
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;

impl TerminalBridge {
    pub(crate) fn adapt_new_terminal() -> TerminalResult<Terminal> {
        Terminal::new(CrosstermBackend::new(stdout()))
            .map_err(|_| TerminalError::CannotConnectStdout)
    }

    #[cfg(target_family = "unix")]
    pub(crate) fn adapt_enter_alternate_screen(&mut self) -> TerminalResult<()> {
        execute!(
            self.raw_mut().backend_mut(),
            EnterAlternateScreen,
            DisableMouseCapture
        )
        .map_err(|_| TerminalError::CannotEnterAlternateMode)
    }

    #[cfg(target_family = "windows")]
    pub(crate) fn adapt_enter_alternate_screen(&mut self) -> TerminalResult<()> {
        use crossterm::{terminal::EnterAlternateScreen, event::DisableMouseCapture};
        use crossterm::execute;
        execute!(
            self.raw_mut().backend_mut(),
            EnterAlternateScreen,
            DisableMouseCapture
        )
        .map_err(|_| TerminalError::CannotEnterAlternateMode)
    }

    #[cfg(target_family = "unix")]
    pub(crate) fn adapt_leave_alternate_screen(&mut self) -> TerminalResult<()> {
        execute!(
            self.raw_mut().backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .map_err(|_| TerminalError::CannotLeaveAlternateMode)
    }

    #[cfg(target_family = "windows")]
    pub(crate) fn adapt_leave_alternate_screen(&mut self) -> TerminalResult<()> {
        use crossterm::{terminal::LeaveAlternateScreen, event::DisableMouseCapture};
        use crossterm::execute;
        execute!(
            self.raw_mut().backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .map_err(|_| TerminalError::CannotLeaveAlternateMode)
    }

    pub(crate) fn adapt_clear_screen(&mut self) -> TerminalResult<()> {
        self.raw_mut()
            .clear()
            .map_err(|_| TerminalError::CannotClear)
    }

    pub(crate) fn adapt_enable_raw_mode(&mut self) -> TerminalResult<()> {
        enable_raw_mode().map_err(|_| TerminalError::CannotToggleRawMode)
    }

    pub(crate) fn adapt_disable_raw_mode(&mut self) -> TerminalResult<()> {
        disable_raw_mode().map_err(|_| TerminalError::CannotToggleRawMode)
    }
}
