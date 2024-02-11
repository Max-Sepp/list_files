use std::{fs, io::ErrorKind, path::Path};

use crate::{ignore_files::IgnoreFiles, output::Output};

fn output_file(output: &mut Box<dyn Output>, path: &Path) {
    output.output(&format!("----- Filename: {} -----\n\n", path.display()));

    let contents = fs::read_to_string(path);

    match contents {
        Ok(contents) => output.output(&contents),
        Err(error) => {
            if error.kind() == ErrorKind::InvalidData {
                output.output("File was not UTF-8 encoded data");
            } else {
                panic!("{}", error);
            }
        }
    };

    output.output("\n");
}

fn list_files_helper(output: &mut Box<dyn Output>, ignore_files: &IgnoreFiles, dir: &Path) {
    let paths = fs::read_dir(dir).unwrap();
    for entry in paths {
        let item = entry.unwrap().path();
        if item.is_dir() {
            list_files_helper(output, ignore_files, &item);
        } else if item.is_file() && ignore_files.check_path(&item) {
            output_file(output, &item);
        }
    }
}

pub fn list_files(output: &mut Box<dyn Output>, ignore_files: &IgnoreFiles) {
    list_files_helper(output, ignore_files, Path::new("./"));
}
