// use colored::*;

// fn main() {
//     println!("{}", "Hello world".blue());
// }

mod ignore_files;
mod list_files;
mod output;

use std::env;

use ignore_files::IgnoreFiles;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Did not enter output type")
    }

    // the first agrument is how the user wants it to be inputted into the system.
    // the second argument is any associated information about the input e.g. file name to output the data to.
    let mut output = output::new(&args[1], args[2..].to_vec());

    // a list of files to check against if the file should be ignored
    let ignore_files = IgnoreFiles::new();

    list_files::list_files(&mut output, &ignore_files);
}
