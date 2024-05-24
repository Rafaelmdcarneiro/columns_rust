use crossterm::{
    cursor::{Hide, Show},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io;

// Failure-guarded setup and cleanup for the terminal.
pub struct TerminalGuard;

impl TerminalGuard {
    pub fn create() -> TerminalGuard {
        let mut stdout = io::stdout();
        enable_raw_mode().unwrap();
        stdout.execute(EnterAlternateScreen).unwrap();
        stdout.execute(Hide).unwrap();
        TerminalGuard
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let mut stdout = io::stdout();
        stdout.execute(LeaveAlternateScreen).unwrap();
        stdout.execute(Show).unwrap();
        disable_raw_mode().unwrap();
    }
}
