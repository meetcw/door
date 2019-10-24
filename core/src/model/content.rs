use crate::entity::ContentEntity;
use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub target: String,
    pub draft: bool,
    pub tags: Vec<String>,
    pub create_time: DateTime<Utc>,
    pub more: Value,
    pub content: String,
    pub path: String,
}

impl From<ContentEntity> for Content {
    fn from(entity: ContentEntity) -> Self {
        Content {
            id: entity.id.clone(),
            title: entity.title.clone(),
            description: entity.description.clone(),
            target: entity.target.clone(),
            draft: entity.draft.clone(),
            tags: entity.tags.clone(),
            create_time: entity.create_time.clone(),
            more: entity.more.clone(),
            content: entity.content.clone(),
            path: entity.path.clone(),
        }
    }
}
