use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::base::file::File;
use crate::base::file_version::FileVersion;
use crate::base::user::User;
use crate::error::Error;
use crate::validation::file::*;
use crate::Result;

use super::user::UserResponse;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FileResponse {
    pub id: String,
    pub owner: String,
    #[validate(custom = "check_filename")]
    pub filename: String,
    #[validate(custom = "check_extension")]
    pub extension: String,
    #[validate(custom = "check_visibility")]
    pub visibility: String,
    #[validate(custom = "check_full_filename")]
    pub full_filename: String,
    #[validate(custom = "check_dir")]
    pub position: String,
    #[validate(custom = "check_fullpath")]
    pub fullpath: String,

    pub created_at: i64,
    pub updated_at: i64,
}

impl TryFrom<File> for FileResponse {
    type Error = Error;
    fn try_from(f: File) -> std::result::Result<Self, Self::Error> {
        let extension = f.extension_to_str().to_string();
        let visibility = f.visibility_to_str().to_string();

        let file_res = Self {
            id: f.id.to_string(),
            owner: f.owner.to_string(),
            filename: f.filename,
            extension,
            visibility,
            full_filename: f.full_filename,
            position: f.position,
            fullpath: f.fullpath,
            created_at: f.created_at,
            updated_at: f.updated_at,
        };
        file_res.validate()?;
        Ok(file_res)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalFileResponse {
    #[serde(flatten)]
    pub file: FileResponse,
    pub owner: UserResponse,
    pub versions: Vec<i64>,
}

impl FinalFileResponse {
    pub fn new(file: File, owner: User, versions: Vec<FileVersion>) -> Result<Self> {
        Ok(Self {
            file: file.into_response()?,
            owner: owner.into_response()?,
            versions: versions
                .into_iter()
                .rev()
                .map(|v| v.version_number)
                .collect(),
        })
    }
}
