use std::io::Write;

use crate::output::Output;

pub struct TerminalOutput {}

impl TerminalOutput {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Output for TerminalOutput {
    fn output(&mut self, string: &str) {
        print!("{}", string);
        std::io::stdout().flush().unwrap();
    }
}
