use std::path::PathBuf;
pub struct Environment {
    pub current_directory: PathBuf,
    pub workspace_directory: PathBuf,
    pub template_directory: PathBuf,
    pub content_directory: PathBuf,
    pub data_directory: PathBuf,
    pub generate_directory: PathBuf,
    pub publish_directory: PathBuf,
}

impl Environment {
    pub fn new(
        current_directory: &str,
        workspace_directory: &str,
    ) -> Environment {
        let current_directory =
            PathBuf::from(current_directory).canonicalize().unwrap();
        let workspace_directory =
            PathBuf::from(workspace_directory).canonicalize().unwrap();
        Environment {
            current_directory: current_directory,
            workspace_directory: workspace_directory.clone(),
            content_directory: workspace_directory.join("content"),
            data_directory: workspace_directory.join("data"),
            template_directory: workspace_directory.join("template"),
            generate_directory: workspace_directory.join("generate"),
            publish_directory: workspace_directory.join("publish"),
        }
    }
}
