use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct SiteEntity {
    #[serde(default = "default_title")]
    pub title: String,
    #[serde(default = "default_subtitle")]
    pub subtitle: String,
    #[serde(default = "default_author")]
    pub author: String,
    #[serde(default = "default_address")]
    pub address: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_more")]
    pub more: Value,
    #[serde(default = "default_theme_directory")]
    pub theme_directory: String,
    #[serde(default = "default_content_directory")]
    pub content_directory: String,
    #[serde(default = "default_data_directory")]
    pub data_directory: String,
    #[serde(default = "default_build_directory")]
    pub build_directory: String,
    #[serde(default = "default_publish_directory")]
    pub publish_directory: String,
    #[serde(skip_serializing, default = "default_workspace")]
    pub root: String,
}

fn default_title() -> String {
    "Title".to_string()
}

fn default_subtitle() -> String {
    "Subtitle".to_string()
}

fn default_author() -> String {
    "Author".to_string()
}

fn default_address() -> String {
    "/".to_string()
}

fn default_theme() -> String {
    "default".to_string()
}

fn default_more() -> Value {
    serde_json::from_str("{}").unwrap()
}

fn default_theme_directory() -> String {
    "theme".to_string()
}

fn default_content_directory() -> String {
    "content".to_string()
}

fn default_build_directory() -> String {
    "build".to_string()
}

fn default_data_directory() -> String {
    "data".to_string()
}

fn default_publish_directory() -> String {
    "publish".to_string()
}

fn default_workspace() -> String {
    ".".to_string()
}
