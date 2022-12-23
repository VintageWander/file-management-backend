use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::base::file::File;
use crate::base::folder::Folder;
use crate::base::user::User;
use crate::error::Error;
use crate::validation::user::check_username;
use crate::Result;

use super::file::FileResponse;
use super::folder::FolderResponse;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: String,
    #[validate(custom = "check_username")]
    pub username: String,

    #[validate(email(message = "The email must be in correct form"))]
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl TryFrom<User> for UserResponse {
    type Error = Error;
    fn try_from(u: User) -> std::result::Result<Self, Self::Error> {
        let res = Self {
            id: u.id.to_string(),
            username: u.username,
            email: u.email,
            created_at: u.created_at,
            updated_at: u.updated_at,
        };

        res.validate()?;
        Ok(res)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalUserResponse {
    #[serde(flatten)]
    pub user: UserResponse,
    pub files: Vec<FileResponse>,
    pub folders: Vec<FolderResponse>,
}

impl FinalUserResponse {
    pub fn new(user: User, files: Vec<File>, folders: Vec<Folder>) -> Result<Self> {
        let files = files
            .into_iter()
            .flat_map(|f| f.into_response())
            .collect::<Vec<_>>();

        let folders = folders
            .into_iter()
            .flat_map(|f| f.into_response())
            .collect::<Vec<_>>();

        Ok(Self {
            user: user.into_response()?,
            files,
            folders,
        })
    }
}
