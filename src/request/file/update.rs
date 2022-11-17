use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    base::file::{File, Visibility},
    validation::file::{check_dir, check_visibility},
    Result,
};
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFileRequest {
    #[validate(custom = "check_dir")]
    pub position: Option<String>,
    #[validate(custom = "check_visibility")]
    pub visibility: String,
}

impl UpdateFileRequest {
    pub fn into_file(self, full_filename: Option<&str>, old_file: File) -> Result<File> {
        self.validate()?;

        let visibility = match self.visibility.as_str() {
            "public" => Visibility::Public,
            "private" => Visibility::Private,
            _ => return Err("Invalid visibility type".into()),
        };

        File::new(
            old_file.id,
            &old_file.owner,
            full_filename.unwrap_or(&old_file.full_filename),
            visibility,
            &self.position.unwrap_or(old_file.position),
            Some(old_file.created_at),
        )
    }
}
