use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::base::file::File;
use crate::base::folder::Folder;
use crate::base::user::User;
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

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id.to_string(),
            username: u.username,
            email: u.email,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

impl UserResponse {
    pub fn from_user(u: User) -> Result<Self> {
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
            .map(|f| f.into_response())
            .collect::<Result<_>>()?;

        let folders = folders
            .into_iter()
            .filter(|f| f.folder_name != user.username)
            .map(|f| f.into_response())
            .collect::<Result<_>>()?;

        Ok(Self {
            user: user.into_response()?,
            files,
            folders,
        })
    }
}
