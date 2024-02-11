mod file_output;
mod terminal_output;

pub trait Output {
    fn output(&mut self, string: &str);
}

pub fn new(variety: &str, rest_args: Vec<String>) -> Box<dyn Output> {
    let result = match variety {
        "terminal" => Box::new(terminal_output::TerminalOutput::new()) as Box<dyn Output>,
        "file" => Box::new(file_output::FileOutput::new(&rest_args[0])) as Box<dyn Output>,
        _ => panic!("Invalid entry of output"),
    };

    return result;
}
