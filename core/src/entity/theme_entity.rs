#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeEntity {
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(skip)]
    pub path: String,
}

fn default_name() -> String {
    String::from("Door")
}
