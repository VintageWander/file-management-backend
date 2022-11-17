use mongodb::bson::oid::ObjectId;

use crate::{
    aws::S3,
    base::folder::Folder,
    db::{file_db::FileDB, folder_db::FolderDB},
    helper::versioning::convert_folder_to_version,
    Result,
};

#[derive(Debug, Clone)]
pub struct FolderService {
    file_db: FileDB,
    folder_db: FolderDB,
    storage: S3,
}

impl FolderService {
    pub fn init(file_db: &FileDB, folder_db: &FolderDB, storage: &S3) -> Self {
        Self {
            file_db: file_db.clone(),
            folder_db: folder_db.clone(),
            storage: storage.clone(),
        }
    }

    pub async fn get_folders_by_owner(&self, owner: &ObjectId) -> Result<Vec<Folder>> {
        self.folder_db.get_folders_by_owner(owner).await
    }

    pub async fn get_public_folders(&self) -> Result<Vec<Folder>> {
        self.folder_db.get_public_folders().await
    }

    pub async fn get_public_folders_by_owner(&self, owner: &ObjectId) -> Result<Vec<Folder>> {
        self.folder_db.get_public_folders_by_owner(owner).await
    }

    pub async fn get_folder_by_id(&self, folder_id: &ObjectId) -> Result<Folder> {
        self.folder_db.get_folder_by_id(folder_id).await
    }

    pub async fn get_public_folder_by_id(&self, folder_id: &ObjectId) -> Result<Folder> {
        self.folder_db.get_public_folder_by_id(folder_id).await
    }

    pub async fn get_folder_by_fullpath(&self, fullpath: &str) -> Result<Folder> {
        self.folder_db.get_folder_by_fullpath(fullpath).await
    }

    pub async fn get_folders_by_prefix_position(&self, prefix: &str) -> Result<Vec<Folder>> {
        self.folder_db.get_folders_by_prefix_position(prefix).await
    }

    pub async fn exists_folder_by_id(&self, folder_id: &ObjectId) -> Result<bool> {
        self.folder_db.exists_folder_by_id(folder_id).await
    }

    pub async fn exists_folder_by_fullpath(&self, fullpath: &str) -> Result<bool> {
        self.folder_db.exists_folder_by_fullpath(fullpath).await
    }

    pub async fn exists_folder_by_position(&self, position: &str) -> Result<bool> {
        self.folder_db.exists_folder_by_position(position).await
    }

    pub async fn create_folder(&self, folder: Folder) -> Result<Folder> {
        if self.exists_folder_by_fullpath(&folder.fullpath).await? {
            return Err(
                "The folder with this name already existed. Please try another folder name".into(),
            );
        }

        if !self.exists_folder_by_position(&folder.position).await? {
            return Err("Cannot create a folder at a virtual position".into());
        }

        let folder = self.folder_db.create_folder(folder).await?;
        let version_folder_fullpath = convert_folder_to_version(&folder.fullpath)?;

        self.storage.create_folder(&folder.fullpath).await?;
        self.storage.create_folder(&version_folder_fullpath).await?;
        Ok(folder)
    }

    pub async fn update_folder_by_id(
        &self,
        folder_id: &ObjectId,
        folder: Folder,
    ) -> Result<Folder> {
        let old_folder = self.folder_db.get_folder_by_id(folder_id).await?;

        if old_folder.fullpath != folder.fullpath {
            let exists_folder = self.exists_folder_by_fullpath(&folder.fullpath).await?;
            let exists_folder_at_postion = self.exists_folder_by_fullpath(&folder.position).await?;
            if exists_folder {
                return Err("This folder name already existed. Please use another name".into());
            }
            if !exists_folder_at_postion {
                return Err("Cannot move folder to a virtual position".into());
            }
            self.folder_db
                .move_inner_folders(
                    &old_folder.position,
                    &old_folder.fullpath,
                    &folder.position,
                    &folder.fullpath,
                )
                .await?;
            self.file_db
                .move_inner_files(
                    &old_folder.position,
                    &old_folder.fullpath,
                    &folder.position,
                    &folder.fullpath,
                )
                .await?;

            self.storage
                .move_folder(&old_folder.fullpath, &folder.fullpath)
                .await?;

            let old_folder_version_path = convert_folder_to_version(&old_folder.fullpath)?;
            let new_folder_version_path = convert_folder_to_version(&folder.fullpath)?;

            self.storage
                .move_folder(&old_folder_version_path, &new_folder_version_path)
                .await?;
        }

        let updated_folder = self.folder_db.update_folder(folder_id, folder).await?;
        Ok(updated_folder)
    }

    pub async fn delete_folder_by_id(&self, folder_id: &ObjectId, owner: &ObjectId) -> Result<()> {
        // Edge case: Cannot let the user delete the root folder
        let folder_to_delete = self.get_folder_by_id(folder_id).await?;
        if folder_to_delete.folder_name == owner.to_string() {
            return Err("You cannot delete the root folder".into());
        }

        let deleted_folder = self
            .folder_db
            .delete_folder_by_id_owner(folder_id, owner)
            .await?;

        let version_folder_fullpath = convert_folder_to_version(&deleted_folder.fullpath)?;

        self.folder_db
            .delete_folders_by_prefix_fullpath(&deleted_folder.fullpath)
            .await?;
        self.folder_db
            .delete_folders_by_prefix_fullpath(&version_folder_fullpath)
            .await?;

        self.file_db
            .delete_files_by_prefix_fullpath(&deleted_folder.fullpath)
            .await?;
        self.file_db
            .delete_files_by_prefix_fullpath(&version_folder_fullpath)
            .await?;

        self.storage.delete_folder(&deleted_folder.fullpath).await?;
        self.storage.delete_folder(&version_folder_fullpath).await?;

        Ok(())
    }

    pub async fn delete_folder_by_owner(&self, owner: &ObjectId) -> Result<()> {
        self.folder_db.delete_folders_by_owner(owner).await
    }
}
