use chrono::{DateTime, Utc};
use serde_json::{self, Value};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentEntity {
    #[serde(default = "default_id")]
    pub id: Uuid,
    #[serde(default = "default_title")]
    pub title: String,
    #[serde(default = "default_description")]
    pub description: String,
    #[serde(default = "default_target")]
    pub target: String,
    #[serde(default = "default_draft")]
    pub draft: bool,
    #[serde(default = "default_tags")]
    pub tags: Vec<String>,
    #[serde(default = "default_create_time")]
    pub create_time: DateTime<Utc>,
    #[serde(default = "default_more")]
    pub more: Value,
    #[serde(skip_deserializing, default = "default_content")]
    pub content: String,
    #[serde(skip)]
    pub path: String,
}

fn default_id() -> Uuid {
    Uuid::new_v4()
}

fn default_title() -> String {
    "TITLE".to_string()
}

fn default_description() -> String {
    "DESCRIPTION".to_string()
}

fn default_target() -> String {
    "POST".to_string()
}

fn default_draft() -> bool {
    true
}

fn default_tags() -> Vec<String> {
    vec![]
}

fn default_create_time() -> DateTime<Utc> {
    Utc::now()
}

fn default_more() -> Value {
    Value::Null
}

fn default_content() -> String {
    "# Content \r\nmarkdown document.".to_string()
}
