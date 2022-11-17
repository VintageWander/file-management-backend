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
        let mut response_files = vec![];
        for file in files {
            response_files.push(file.into_response()?)
        }

        let mut response_folders = vec![];
        for folder in folders {
            response_folders.push(folder.into_response()?)
        }

        Ok(Self {
            user: user.into_response()?,
            files: response_files,
            folders: response_folders,
        })
    }
}
