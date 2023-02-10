use std::{collections::HashMap, str::FromStr};

use futures::try_join;
use mongodb::bson::{oid::ObjectId, Bson, Document};

use crate::{
    aws::S3,
    base::folder::Folder,
    db::{file_db::FileDB, folder_db::FolderDB},
    helper::into_string,
    validation::file::check_dir,
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

    pub async fn get_folders_by_map(&self, map: &HashMap<String, String>) -> Result<Vec<Folder>> {
        let mut document = HashMap::new();
        for i in map {
            if *i.0 == "owner" {
                document.insert(
                    "owner".to_string(),
                    Bson::ObjectId(ObjectId::from_str(i.1)?),
                );
            } else {
                document.insert(i.0.to_string(), Bson::String(i.1.to_string()));
            }
        }
        let doc = Document::from_iter(document);
        self.folder_db.get_folders_by(doc).await
    }

    pub async fn get_folders_by_owner(&self, owner: &ObjectId) -> Result<Vec<Folder>> {
        self.folder_db.get_folders_by_owner(owner).await
    }

    // pub async fn get_public_folders(&self) -> Result<Vec<Folder>> {
    //     self.folder_db.get_public_folders().await
    // }

    pub async fn get_public_folders_by_owner(&self, owner: &ObjectId) -> Result<Vec<Folder>> {
        self.folder_db.get_public_folders_by_owner(owner).await
    }

    pub async fn get_folder_by_id(&self, folder_id: &ObjectId) -> Result<Folder> {
        self.folder_db.get_folder_by_id(folder_id).await
    }

    // pub async fn get_folder_by_id_owner(
    //     &self,
    //     folder_id: &ObjectId,
    //     owner: &ObjectId,
    // ) -> Result<Folder> {
    //     self.folder_db
    //         .get_folder_by_id_owner(folder_id, owner)
    //         .await
    // }

    pub async fn get_public_folder_by_id(&self, folder_id: &ObjectId) -> Result<Folder> {
        self.folder_db.get_public_folder_by_id(folder_id).await
    }

    // pub async fn get_folder_by_fullpath(&self, fullpath: &str) -> Result<Folder> {
    //     check_dir(fullpath).map_err(into_string)?;
    //     self.folder_db.get_folder_by_fullpath(fullpath).await
    // }

    // pub async fn get_folders_by_prefix_position(&self, prefix: &str) -> Result<Vec<Folder>> {
    //     check_dir(prefix).map_err(into_string)?;
    //     self.folder_db.get_folders_by_prefix_position(prefix).await
    // }

    // pub async fn get_folders_by_prefix_exact_position(&self, prefix: &str) -> Result<Vec<Folder>> {
    //     check_dir(prefix).map_err(into_string)?;
    //     self.folder_db
    //         .get_folders_by_prefix_exact_position(prefix)
    //         .await
    // }

    // pub async fn get_public_folders_by_prefix_position(&self, prefix: &str) -> Result<Vec<Folder>> {
    //     check_dir(prefix).map_err(into_string)?;
    //     self.folder_db
    //         .get_public_folders_by_prefix_position(prefix)
    //         .await
    // }

    // pub async fn exists_folder_by_id(&self, folder_id: &ObjectId) -> Result<bool> {
    //     self.folder_db.exists_folder_by_id(folder_id).await
    // }

    pub async fn exists_folder_by_fullpath(&self, fullpath: &str) -> Result<bool> {
        check_dir(fullpath).map_err(into_string)?;
        self.folder_db.exists_folder_by_fullpath(fullpath).await
    }

    // pub async fn exists_folder_by_position(&self, position: &str) -> Result<bool> {
    //     check_dir(position).map_err(into_string)?;
    //     self.folder_db.exists_folder_by_position(position).await
    // }

    pub async fn create_folder(&self, folder: Folder) -> Result<Folder> {
        if self.exists_folder_by_fullpath(&folder.fullpath).await? {
            return Err(
                "The folder with this name already existed. Please try another folder name".into(),
            );
        }

        if !self.exists_folder_by_fullpath(&folder.position).await? {
            return Err("Cannot create a folder at a virtual position".into());
        }

        let folder = self.folder_db.create_folder(folder).await?;

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
            if old_folder.fullpath == folder.position {
                return Err("Cannot move to self".into());
            }
            if old_folder.fullpath.matches('/').count() == folder.fullpath.matches('/').count() {
                // This means that the user is renaming a folder
                try_join!(
                    self.folder_db.move_inner_folders(
                        &old_folder.fullpath,
                        &old_folder.fullpath,
                        &folder.fullpath,
                    ),
                    self.file_db.move_inner_files(
                        &old_folder.fullpath,
                        &old_folder.fullpath,
                        &folder.fullpath
                    )
                )?;
            } else {
                // This indicates a move folder to new position
                try_join!(
                    self.folder_db.move_inner_folders(
                        &old_folder.position,
                        &old_folder.fullpath,
                        &folder.position, // the difference is here
                    ),
                    self.file_db.move_inner_files(
                        &old_folder.position,
                        &old_folder.fullpath,
                        &folder.position
                    )
                )?;
            }
        }

        let updated_folder = self.folder_db.update_folder(folder_id, folder).await?;
        Ok(updated_folder)
    }

    pub async fn delete_folder_by_id_owner(
        &self,
        folder_id: &ObjectId,
        owner: &ObjectId,
    ) -> Result<()> {
        // Edge case: Cannot let the user delete the root folder
        let folder_to_delete = self.get_folder_by_id(folder_id).await?;
        if folder_to_delete.folder_name == owner.to_string() {
            return Err("You cannot delete the root folder".into());
        }

        let deleted_folder = self
            .folder_db
            .delete_folder_by_id_owner(folder_id, owner)
            .await?;

        // Delete all of the files in S3 that are related to the folder
        let files = self
            .file_db
            .get_files_by_prefix_fullpath(&deleted_folder.fullpath)
            .await?;

        for file in files {
            let internal_full_filename = &format!("{}.{}", file.id, file.extension_to_str());
            let internal_file_version_path = &format!("{}/", file.id);

            self.storage.delete_file(internal_full_filename).await?;
            self.storage
                .delete_folder(internal_file_version_path)
                .await?;
        }

        self.folder_db
            .delete_folders_by_prefix_fullpath(&deleted_folder.fullpath)
            .await?;

        self.file_db
            .delete_files_by_prefix_fullpath(&deleted_folder.fullpath)
            .await?;

        Ok(())
    }
}
