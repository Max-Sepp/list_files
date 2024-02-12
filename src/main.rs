// use colored::*;

// fn main() {
//     println!("{}", "Hello world".blue());
// }

mod ignore_files;
mod list_files;
mod output;

use clap::{Parser, Subcommand};
use output::{file_output::FileOutput, terminal_output::TerminalOutput, Output};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    File { output_path: String },
    Terminal,
}

fn main() {
    let args = Args::parse();

    let mut output: Box<dyn Output>;

    let output_path = match args.cmd {
        Commands::File { output_path } => {
            output = Box::new(FileOutput::new(&output_path));
            Some(output_path)
        }
        Commands::Terminal => {
            output = Box::new(TerminalOutput::new());
            None
        }
    };

    // a list of files to check against if the file should be ignored
    let ignore_files = ignore_files::IgnoreFiles::new(output_path);

    list_files::list_files(&mut output, &ignore_files);
}
