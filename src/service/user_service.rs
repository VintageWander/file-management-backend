use mongodb::bson::oid::ObjectId;

use crate::{
    aws::S3,
    base::{folder::Folder, user::User},
    db::{file_db::FileDB, file_version_db::FileVersionDB, folder_db::FolderDB, user_db::UserDB},
    Result,
};

#[derive(Debug, Clone)]
pub struct UserService {
    user_db: UserDB,
    file_db: FileDB,
    folder_db: FolderDB,
    file_version_db: FileVersionDB,
    storage: S3,
}

impl UserService {
    pub fn init(
        user_db: &UserDB,
        file_db: &FileDB,
        folder_db: &FolderDB,
        file_verion_db: &FileVersionDB,
        storage: &S3,
    ) -> Self {
        Self {
            user_db: user_db.clone(),
            file_db: file_db.clone(),
            folder_db: folder_db.clone(),
            file_version_db: file_verion_db.clone(),
            storage: storage.clone(),
        }
    }

    pub async fn get_users(&self) -> Result<Vec<User>> {
        self.user_db.get_users().await
    }

    pub async fn get_user_by_id(&self, user_id: &ObjectId) -> Result<User> {
        self.user_db.get_user_by_id(user_id).await
    }

    pub async fn get_user_by_login_info(&self, username: &str, password: &str) -> Result<User> {
        self.user_db
            .get_user_by_login_info(username, password)
            .await
    }

    // pub async fn exists_user_by_id(&self, user_id: &ObjectId) -> Result<bool> {
    //     self.user_db.exists_user_by_id(user_id).await
    // }

    pub async fn exists_user_by_username(&self, username: &str) -> Result<bool> {
        self.user_db.exists_user_by_username(username).await
    }

    pub async fn exists_user_by_email(&self, email: &str) -> Result<bool> {
        self.user_db.exists_user_by_email(email).await
    }

    // pub async fn exists_user_by_login_info(&self, username: &str, password: &str) -> Result<bool> {
    //     self.user_db
    //         .exists_user_by_login_info(username, password)
    //         .await
    // }

    pub async fn create_user(&self, user: User) -> Result<User> {
        if self.exists_user_by_email(&user.email).await? {
            return Err(
                "There is a user associated with this email. Please use another email".into(),
            );
        }

        if self.exists_user_by_username(&user.username).await? {
            return Err(
                "There is a user associated with this username. Please pick another username"
                    .into(),
            );
        }

        let new_user = self.user_db.create_user(user).await?;
        let root_folder = Folder::new_root(&new_user)?;

        self.folder_db.create_folder(root_folder).await?;

        Ok(new_user)
    }

    pub async fn update_user(&self, user: User) -> Result<User> {
        let old_user = self.get_user_by_id(&user.id).await?;
        if old_user.username != user.username
            && self.exists_user_by_username(&user.username).await?
        {
            return Err("This username is already taken. Please try another username".into());
        }

        if old_user.email != user.email && self.exists_user_by_email(&user.email).await? {
            return Err("This email is already taken. Please try another email".into());
        }

        let updated_user = self.user_db.update_user(&user.id.clone(), user).await?;

        let user_files = self.file_db.get_files_by_owner(&updated_user.id).await?;
        for file in user_files {
            let (_, rest) = file
                .position
                .split_once('/')
                .ok_or("Cannot get the rest file path to update")?;

            let new_position = &format!("{}/{}", updated_user.username, rest);
            self.file_db
                .move_inner_files(&file.position, &file.fullpath, new_position)
                .await?;
        }

        let user_folders = self
            .folder_db
            .get_folders_by_owner(&updated_user.id)
            .await?;
        for folder in user_folders {
            let (_, rest) = folder
                .position
                .split_once('/')
                .ok_or("Cannot get the rest folder path to update")?;

            let new_position = &format!("{}/{}", updated_user.username, rest);

            self.folder_db
                .update_folder(
                    &folder.id,
                    Folder {
                        folder_name: updated_user.username.clone(),
                        ..folder.clone()
                    },
                )
                .await?;
            self.folder_db
                .move_inner_folders(&folder.position, &folder.fullpath, new_position)
                .await?;
        }

        Ok(updated_user)
    }

    pub async fn update_refresh_token(
        &self,
        user_id: &ObjectId,
        refresh_token: &str,
    ) -> Result<()> {
        self.user_db
            .update_refresh_token(user_id, refresh_token)
            .await
    }

    pub async fn delete_user_by_id(&self, user_id: &ObjectId) -> Result<()> {
        let deleted_user = self.user_db.delete_user(user_id).await?;

        let files = self.file_db.get_files_by_owner(&deleted_user.id).await?;

        for file in files {
            let internal_full_filename = &format!("{}.{}", file.id, file.extension_to_str());
            let internal_file_version_path = &format!("{}/", file.id);

            self.file_version_db
                .delete_versions_by_file_id(&file.id)
                .await?;

            self.storage.delete_file(internal_full_filename).await?;
            self.storage
                .delete_folder(internal_file_version_path)
                .await?;
        }

        self.file_db.delete_files_by_owner(&deleted_user.id).await?;
        self.folder_db
            .delete_folders_by_owner(&deleted_user.id)
            .await?;

        Ok(())
    }
}
