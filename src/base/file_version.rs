use chrono::Utc;
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::file::File;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FileVersion {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub file: ObjectId,
    pub version_number: i64,

    pub created_at: i64,
    pub updated_at: i64,
}

impl From<FileVersion> for Document {
    fn from(f: FileVersion) -> Self {
        doc! {
            "file": f.file,
            "versionNumber": f.version_number,
            "createdAt": f.created_at,
            "updatedAt": f.updated_at,
        }
    }
}

impl FileVersion {
    pub fn new(file: &File, version_number: i64, created_at: Option<i64>) -> Self {
        Self {
            id: ObjectId::new(),
            file: file.id,
            version_number,
            created_at: created_at.unwrap_or_else(|| Utc::now().timestamp_millis()),
            updated_at: Utc::now().timestamp_millis(),
        }
    }
}
