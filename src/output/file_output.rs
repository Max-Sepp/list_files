use crate::output::Output;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

pub struct FileOutput {
    file: File,
}

impl FileOutput {
    pub fn new(path: &str) -> Self {
        if Path::new(path).exists() {
            let data_file = OpenOptions::new()
                .append(true)
                .open(path)
                .expect("Cannot open file");

            return Self { file: data_file };
        } else {
            let data_file = File::create(path).expect("Cannot create file");

            return Self { file: data_file };
        }
    }
}

impl Output for FileOutput {
    fn output(&mut self, string: &str) {
        let result = self.file.write(string.as_bytes());
        match result {
            Ok(_) => {}
            Err(error) => panic!("Problem writing to file: {}", error),
        };
    }
}
