mod ignore_files;
mod list_files;
mod output;

use std::path::PathBuf;

use clap::{arg, command, value_parser};
use output::{file_output::FileOutput, terminal_output::TerminalOutput, Output};

fn main() {
    let args = command!()
        .arg(
            arg!(-o --output <FILE> "Specify an output file")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let mut output: Box<dyn Output>;

    // if this is Some then user wants to output to a file otherwise just print to a terminal
    let output_path = match args.get_one::<PathBuf>("output") {
        Some(output_path) => {
            output = Box::new(FileOutput::new(output_path));
            output_path.as_os_str().to_str()
        }
        None => {
            output = Box::new(TerminalOutput::new());
            None
        }
    };

    // a list of files to check against if the file should be ignored
    let ignore_files = ignore_files::IgnoreFiles::new(output_path);

    list_files::list_files(&mut output, &ignore_files);
}
