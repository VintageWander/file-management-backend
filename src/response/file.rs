use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::base::file::File;
use crate::validation::file::*;
use crate::Result;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
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

impl FileResponse {
    pub fn from_file(f: File) -> Result<Self> {
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
