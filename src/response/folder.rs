use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    base::{folder::Folder, user::User},
    validation::file::{check_dir, check_folder_name, check_visibility},
    Result,
};

use super::user::UserResponse;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FolderResponse {
    pub id: String,

    pub owner: String,

    #[validate(custom = "check_folder_name")]
    pub folder_name: String,

    #[validate(custom = "check_visibility")]
    pub visibility: String,

    #[validate(custom = "check_dir")]
    pub position: String,
    #[validate(custom = "check_dir")]
    pub fullpath: String,

    pub created_at: i64,
    pub updated_at: i64,
}

impl FolderResponse {
    pub fn from_folder(f: Folder) -> Result<Self> {
        let visibility = f.visibility_to_str().to_string();

        let folder_res = Self {
            id: f.id.to_string(),
            owner: f.owner.to_string(),
            folder_name: f.folder_name,
            visibility,
            position: f.position,
            fullpath: f.fullpath,
            created_at: f.created_at,
            updated_at: f.updated_at,
        };

        folder_res.validate()?;
        Ok(folder_res)
    }
}

#[derive(Serialize, Deserialize)]
pub struct FinalFolderResponse {
    #[serde(flatten)]
    pub folder: FolderResponse,
    pub owner: UserResponse,
}

impl FinalFolderResponse {
    pub fn new(folder: Folder, owner: User) -> Result<Self> {
        Ok(Self {
            folder: folder.into_response()?,
            owner: owner.into_response()?,
        })
    }
}
