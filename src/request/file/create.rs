use crate::{
    base::{
        file::{File, Visibility},
        user::User,
    },
    helper::into_string,
    validation::file::{check_dir, check_full_filename, check_visibility},
    Result,
};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateFileRequest {
    #[validate(custom = "check_dir")]
    pub position: String,
    #[validate(custom = "check_visibility")]
    pub visibility: String,
}

impl CreateFileRequest {
    pub fn into_file(self, owner: &User, full_filename: &str) -> Result<File> {
        self.validate()?;
        check_full_filename(full_filename).map_err(into_string)?;

        let visibility = match self.visibility.as_str() {
            "public" => Visibility::Public,
            "private" => Visibility::Private,
            _ => return Err("Invalid visibility type".into()),
        };

        let file = File::new(
            ObjectId::new(),
            owner,
            full_filename,
            visibility,
            &self.position,
            None,
        )?;
        Ok(file)
    }
}
