use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::helper::into_string;
use crate::response::folder::FolderResponse;
use crate::validation::file::{check_dir, check_folder_name};
use crate::Result;

use super::user::User;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    pub owner: ObjectId,

    #[validate(custom = "check_folder_name")]
    pub folder_name: String,

    pub visibility: Visibility,

    #[validate(custom = "check_dir")]
    pub position: String,
    #[validate(custom = "check_dir")]
    pub fullpath: String,

    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

impl From<Folder> for Document {
    fn from(f: Folder) -> Self {
        let visibility = f.visibility_to_str();

        doc! {
            "visibility": visibility,
            "owner": f.owner,
            "folderName": f.folder_name,
            "position": f.position,
            "fullpath": f.fullpath,
            "createdAt": f.created_at,
            "updatedAt": f.updated_at,
        }
    }
}

impl Folder {
    pub fn new(
        id: ObjectId,
        owner: &User,
        folder_name: &str,
        visibility: Visibility,
        position: &str,
        created_at: Option<i64>,
    ) -> Result<Self> {
        check_folder_name(folder_name).map_err(into_string)?;
        check_dir(position).map_err(into_string)?;

        let position = format!("{}/{position}", owner.username);
        let fullpath = format!("{position}{folder_name}/");
        let folder = Folder {
            id,
            owner: owner.id,
            visibility,
            folder_name: folder_name.to_string(),
            position,
            fullpath,
            created_at: created_at.unwrap_or_else(|| Utc::now().timestamp_millis()),
            updated_at: Utc::now().timestamp_millis(),
        };
        folder.validate()?;
        Ok(folder)
    }

    pub fn into_response(self) -> Result<FolderResponse> {
        FolderResponse::from_folder(self)
    }

    pub fn new_root(owner: &User) -> Result<Self> {
        let folder_id = ObjectId::new();
        let root = Self {
            id: folder_id,
            owner: owner.id,
            folder_name: owner.username.clone(),
            visibility: Visibility::Private,
            position: format!("{}/", owner.username),
            fullpath: format!("{}/", owner.username),
            created_at: Utc::now().timestamp_millis(),
            updated_at: Utc::now().timestamp_millis(),
        };
        root.validate()?;
        Ok(root)
    }

    pub fn visibility_to_str(&self) -> &str {
        match self.visibility {
            Visibility::Public => "public",
            Visibility::Private => "private",
        }
    }
}
