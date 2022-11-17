use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    base::folder::{Folder, Visibility},
    validation::file::{check_dir, check_folder_name, check_visibility},
    Result,
};

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFolderRequest {
    #[validate(custom = "check_folder_name")]
    pub folder_name: Option<String>,
    #[validate(custom = "check_visibility")]
    pub visibility: Option<String>,
    #[validate(custom = "check_dir")]
    pub position: String,
}

impl UpdateFolderRequest {
    pub fn into_folder(self, owner: &ObjectId, old_folder: Folder) -> Result<Folder> {
        self.validate()?;

        let visibility = match self.visibility {
            Some(v) => match v.as_str() {
                "public" => Visibility::Public,
                "private" => Visibility::Private,
                _ => return Err("Invalid visibility type".into()),
            },
            None => old_folder.visibility,
        };

        Folder::new(
            ObjectId::new(),
            owner,
            &self.folder_name.unwrap_or(old_folder.folder_name),
            visibility,
            &self.position,
            Some(old_folder.created_at),
        )
    }
}
