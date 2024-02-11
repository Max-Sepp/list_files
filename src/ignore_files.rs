use std::{fs::read_to_string, path::Path};

pub struct IgnoreFiles {
    files_to_ignore: Vec<String>,
}

impl IgnoreFiles {
    pub fn new() -> Self {
        let mut files_to_ignore = vec![".git".to_string()];

        let gitignore_path = Path::new("./.gitignore");

        if gitignore_path.exists() {
            for line in read_to_string(gitignore_path).unwrap().lines() {
                files_to_ignore.push(line.to_owned());
            }
        }

        return Self { files_to_ignore };
    }

    pub fn check_path(&self, path: &Path) -> bool {
        let path_string = path.to_str().expect("Not valid path");

        for ignore in &self.files_to_ignore {
            if path_string.contains(ignore) {
                return false;
            }
        }

        return true;
    }
}
