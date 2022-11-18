use serde::{Deserialize, Serialize};

use crate::{
    base::{file::File, folder::Folder, user::User},
    Result,
};

use self::{file::FileResponse, folder::FolderResponse, user::UserResponse};

pub mod file;
pub mod folder;
pub mod user;

#[derive(Deserialize, Serialize)]
pub struct Response {
    #[serde(flatten)]
    pub user: UserResponse,
    pub files: Vec<FileResponse>,
    pub folders: Vec<FolderResponse>,
}

impl Response {
    pub fn new(user: User, files: Vec<File>, folders: Vec<Folder>) -> Result<Self> {
        let files = files
            .into_iter()
            .map(|f| f.into_response())
            .collect::<Result<_>>()?;

        let folders = folders
            .into_iter()
            .map(|f| f.into_response())
            .collect::<Result<_>>()?;

        Ok(Self {
            user: user.into_response()?,
            files,
            folders,
        })
    }
}
