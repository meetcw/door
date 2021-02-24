use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct SiteEntity {
    #[serde(default = "default_title")]
    pub title: String,
    #[serde(default = "default_description")]
    pub description: String,
    #[serde(default = "default_author")]
    pub author: String,
    #[serde(default = "default_address")]
    pub address: String,
    #[serde(default = "default_template")]
    pub template: String,
    #[serde(default = "default_more")]
    pub more: Value,
}

fn default_title() -> String {
    "Site Title".to_string()
}

fn default_description() -> String {
    "Description".to_string()
}

fn default_author() -> String {
    "Author".to_string()
}

fn default_address() -> String {
    "/".to_string()
}

fn default_template() -> String {
    "default".to_string()
}

fn default_more() -> Value {
    serde_json::from_str("{}").unwrap()
}
