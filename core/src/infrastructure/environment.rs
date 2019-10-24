use std::fs;
use std::path::Path;

pub struct Environment {
    pub current_directory: String,
    pub workspace: String,
}

impl Environment {
    pub fn new(current_directory: &str, workspace: &str) -> Environment {
        Environment {
            current_directory: current_directory.to_string(),
            workspace: workspace.to_string(),
        }
    }
}
