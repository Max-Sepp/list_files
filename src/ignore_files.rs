use std::{fs::read_to_string, path::Path};

pub struct IgnoreFiles {
    files_to_ignore: Vec<String>,
}

impl IgnoreFiles {
    pub fn new(output_path: Option<String>) -> Self {
        let mut files_to_ignore = vec![".git".to_string()];

        if output_path.is_some() {
            files_to_ignore.push(output_path.unwrap())
        }

        let gitignore_path = Path::new("./.gitignore");

        if gitignore_path.exists() {
            for line in read_to_string(gitignore_path).unwrap().lines() {
                match line.chars().nth(0) {
                    None => {}
                    Some(first_char) => {
                        if first_char != '#' {
                            files_to_ignore.push(line.to_owned());
                        }
                    }
                }
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
