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
pub struct CreateFolderRequest {
    #[validate(custom = "check_folder_name")]
    pub folder_name: String,
    #[validate(custom = "check_visibility")]
    pub visibility: String,
    #[validate(custom = "check_dir")]
    pub position: String,
}

impl CreateFolderRequest {
    pub fn into_folder(self, owner: &ObjectId) -> Result<Folder> {
        self.validate()?;

        let visibility = match self.visibility.as_str() {
            "public" => Visibility::Public,
            "private" => Visibility::Private,
            _ => return Err("Invalid visibility".into()),
        };

        Folder::new(
            ObjectId::new(),
            owner,
            &self.folder_name,
            visibility,
            &self.position,
            None,
        )
    }
}
