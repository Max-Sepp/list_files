pub mod file_output;
pub mod terminal_output;

pub trait Output {
    fn output(&mut self, string: &str);
}
