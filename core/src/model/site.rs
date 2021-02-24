use crate::entity::SiteEntity;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Site {
    pub title: String,
    pub author: String,
    pub description: String,
    pub address: String,
    pub template: String,
    pub more: Value,
}

impl From<SiteEntity> for Site {
    fn from(entity: SiteEntity) -> Self {
        Site {
            title: entity.title,
            author: entity.author,
            description: entity.description,
            address: entity.address,
            template: entity.template,
            more: entity.more,
        }
    }
}
