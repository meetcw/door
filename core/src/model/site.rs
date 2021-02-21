use crate::entity::SiteEntity;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Site {
    pub title: String,
    pub author: String,
    pub description: String,
    pub address: String,
    pub theme: String,
    pub more: Value,
    pub theme_directory: String,
    pub content_directory: String,
    pub data_directory: String,
    pub build_directory: String,
    pub publish_directory: String,
    pub root: String,
}

impl From<SiteEntity> for Site {
    fn from(entity: SiteEntity) -> Self {
        Site {
            title: entity.title,
            author: entity.author,
            description: entity.description,
            address: entity.address,
            theme: entity.theme,
            more: entity.more,
            theme_directory: entity.theme_directory,
            content_directory: entity.content_directory,
            data_directory: entity.data_directory,
            build_directory: entity.build_directory,
            publish_directory: entity.publish_directory,
            root: entity.root,
        }
    }
}
