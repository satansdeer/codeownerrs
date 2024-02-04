pub mod paths {
    use ignore::{DirEntry, WalkBuilder};

    pub fn list(depth: Option<usize>) -> Vec<DirEntry> {
        WalkBuilder::new("./")
            .max_depth(depth)
            .build()
            .filter_map(|entry| entry.ok())
            .collect()
    }
}

pub mod test_utils {
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    /// Creates a temporary file with the given contents and returns the path.
    /// The temporary file is automatically deleted when it goes out of scope.
    pub fn create_temp_codeowners_file(contents: &str) -> PathBuf {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        writeln!(temp_file, "{}", contents).expect("Failed to write to temporary file");

        temp_file.into_temp_path().to_path_buf()
    }
}

pub mod code_owners {
    use std::fs::File;
    use std::io::prelude::*;

    /// Represents an entry in the code owners file.
    ///
    /// Each entry consists of a path and a list of owners (usernames).
    pub struct Entry {
        path: String,
        owners: Vec<String>,
    }

    /// Manages the mapping from file paths to code owners.
    ///
    /// # Examples
    ///
    /// ```
    /// use codeownerrs::code_owners::CodeOwners;
    /// let code_owners = CodeOwners::new("CODEOWNERS".to_string()).unwrap();
    /// ```
    pub struct CodeOwners {
        pub entries: Vec<Entry>,
    }

    impl CodeOwners {
        /// Creates a new `CodeOwners` instance from a given file.
        ///
        /// The file should be formatted with each line containing a path followed by
        /// space-separated owner names.
        ///
        /// # Arguments
        ///
        /// * `file` - A string slice that holds the name of the file
        ///
        /// # Examples
        ///
        /// ```
        /// use codeownerrs::code_owners::CodeOwners;
        /// use codeownerrs::test_utils::create_temp_codeowners_file;
        ///
        /// let contents = "/src user1 user2\n";
        /// let temp_path = create_temp_codeowners_file(contents);
        /// let code_owners = CodeOwners::new(temp_path.to_string()).unwrap();
        /// assert_eq!(code_owners.entries.len(), 1);
        /// ```
        ///
        /// # Errors
        ///
        /// This function will return an error if the file cannot be opened or read.
        pub fn new(file: String) -> std::io::Result<CodeOwners> {
            let mut file = File::open(file)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let entries = contents
                .split('\n') // Correct split pattern for newline
                .filter_map(|line| {
                    let mut parts = line.split_whitespace().collect::<Vec<&str>>();
                    if parts.len() < 2 {
                        return None; // Skip lines that don't have at least one path and one owner
                    }
                    let path = parts.remove(0).to_string();
                    let owners = parts.into_iter().map(|s| s.to_string()).collect();
                    Some(Entry { path, owners })
                })
                .collect();
            Ok(CodeOwners { entries })
        }

        /// Retrieves the owners for a given file path.
        ///
        /// # Arguments
        ///
        /// * `path` - A string slice that holds the path to look up
        ///
        /// # Examples
        ///
        /// Assuming `code_owners` has been properly initialized:
        ///
        /// ```
        /// # use codeownerrs::code_owners::CodeOwners;
        /// # let code_owners = CodeOwners::new("path/to/codeowners/file.txt".to_string()).unwrap();
        /// let owners = code_owners.get_owners("/src/lib.rs");
        /// assert!(owners.contains(&"username1".to_string()));
        /// ```
        ///
        /// # Panics
        ///
        /// This function will panic if the path is not found in any entry.
        pub fn get_owners(&self, path: &str) -> &Vec<String> {
            &self
                .entries
                .iter()
                .find(|e| e.path.starts_with(path))
                .expect("Path not found")
                .owners
        }
    }
}
