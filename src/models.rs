use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Url {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub long_url: String,
    pub short_code: String,
    pub clicks: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUrlRequest {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUrlResponse {
    pub short_url: String,
    pub original_url: String,
} 