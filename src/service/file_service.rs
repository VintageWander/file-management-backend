use std::{collections::HashMap, str::FromStr};

use chrono::Utc;
use mongodb::bson::{oid::ObjectId, Bson, Document};

use crate::{
    aws::S3,
    base::{file::File, file_version::FileVersion},
    db::{file_db::FileDB, file_version_db::FileVersionDB, folder_db::FolderDB},
    helper::into_string,
    validation::file::{check_dir, check_fullpath},
    Result,
};

#[derive(Debug, Clone)]
pub struct FileService {
    file_db: FileDB,
    folder_db: FolderDB,
    version_db: FileVersionDB,
    storage: S3,
}

impl FileService {
    pub fn init(
        file_db: &FileDB,
        folder_db: &FolderDB,
        version_db: &FileVersionDB,
        storage: &S3,
    ) -> Self {
        Self {
            file_db: file_db.clone(),
            folder_db: folder_db.clone(),
            storage: storage.clone(),
            version_db: version_db.clone(),
        }
    }

    pub async fn get_files_by_map(&self, map: &HashMap<String, String>) -> Result<Vec<File>> {
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
        self.file_db.get_files_by(doc).await
    }

    pub async fn get_files_by_owner(&self, owner: &ObjectId) -> Result<Vec<File>> {
        self.file_db.get_files_by_owner(owner).await
    }

    pub async fn get_public_files_by_owner(&self, owner: &ObjectId) -> Result<Vec<File>> {
        self.file_db.get_public_files_by_owner(owner).await
    }

    pub async fn get_public_files(&self) -> Result<Vec<File>> {
        self.file_db.get_public_files().await
    }

    pub async fn get_file_by_id(&self, file_id: &ObjectId) -> Result<File> {
        self.file_db.get_file_by_id(file_id).await
    }

    pub async fn get_public_file_by_id(&self, file_id: &ObjectId) -> Result<File> {
        self.file_db.get_public_file_by_id(file_id).await
    }

    pub async fn get_file_by_id_owner(&self, file_id: &ObjectId, owner: &ObjectId) -> Result<File> {
        self.file_db.get_file_by_id_owner(file_id, owner).await
    }

    pub async fn get_files_by_prefix_position(&self, prefix: &str) -> Result<Vec<File>> {
        check_dir(prefix).map_err(into_string)?;
        self.file_db.get_files_by_prefix_position(prefix).await
    }

    pub async fn get_files_by_prefix_exact_position(&self, prefix: &str) -> Result<Vec<File>> {
        check_dir(prefix).map_err(into_string)?;
        self.file_db
            .get_files_by_prefix_exact_position(prefix)
            .await
    }

    pub async fn get_public_files_by_prefix_position(&self, prefix: &str) -> Result<Vec<File>> {
        check_dir(prefix).map_err(into_string)?;
        self.file_db
            .get_public_files_by_prefix_position(prefix)
            .await
    }

    pub async fn exists_file_by_id(&self, file_id: &ObjectId) -> Result<bool> {
        self.file_db.exists_file_by_id(file_id).await
    }

    pub async fn exists_file_by_fullpath(&self, fullpath: &str) -> Result<bool> {
        check_fullpath(fullpath).map_err(into_string)?;
        self.file_db.exists_file_by_fullpath(fullpath).await
    }

    pub async fn create_file(&self, file: File, data: Vec<u8>) -> Result<File> {
        let exists_file = self.exists_file_by_fullpath(&file.fullpath).await?;
        let exists_position = self
            .folder_db
            .exists_folder_by_fullpath(&file.position)
            .await?;
        if exists_file {
            return Err(
                "The file with this name already existed in this path. Please try another name"
                    .into(),
            );
        }
        if !exists_position {
            return Err("Cannot create a file at a virtual position".into());
        }
        if !data.is_empty() {
            let internal_full_filename = &format!("{}.{}", file.id, file.extension_to_str());

            self.storage
                .create_file(internal_full_filename, data)
                .await?;
            self.storage.create_folder(&format!("{}/", file.id)).await?;
        }

        let file = self.file_db.create_file(file).await?;
        Ok(file)
    }

    pub async fn update_file_by_id(
        &self,
        file_id: &ObjectId,
        file: File,
        data: Vec<u8>,
    ) -> Result<File> {
        let old_file = self.get_file_by_id(file_id).await?;
        if old_file.extension != file.extension {
            return Err(
                "Changing extension is not supported, as it might render the file unusable".into(),
            );
        }

        if old_file.fullpath != file.fullpath {
            let exists_file = self.file_db.exists_file_by_fullpath(&file.fullpath).await?;
            let exists_position = self
                .folder_db
                .exists_folder_by_fullpath(&file.position)
                .await?;

            if exists_file {
                return Err(
                    "There's a file with a same name at this position. Please try another name"
                        .into(),
                );
            }

            if !exists_position {
                return Err("Cannot move file to a virtual location".into());
            }
        }

        if !data.is_empty() {
            // Create a version number
            let version = Utc::now().timestamp_millis();

            // Get the file path on the versioning side

            let file_version_path = format!("{}/{}.{}", file.id, version, file.extension_to_str());

            self.version_db
                .create_version_with_file_id(FileVersion::new(&file, version, None))
                .await?;

            let internal_full_filename = &format!("{}.{}", file.id, file.extension_to_str());

            // Move the old file to there
            self.storage
                .move_file(internal_full_filename, &file_version_path)
                .await?;

            // Create a new file at the previous path
            self.storage
                .create_file(internal_full_filename, data)
                .await?;
        }

        let updated_file = self.file_db.update_file_by_id(file_id, file).await?;
        Ok(updated_file)
    }

    pub async fn restore_file_from_version(
        &self,
        file_id: &ObjectId,
        owner: &ObjectId,
        version: i64,
    ) -> Result<File> {
        // The restore mechanism will basically does a swap
        // Before getting the file to be replaced, let's check if the version exists or not
        if !self
            .version_db
            .exists_version_by_file_id_version(file_id, version)
            .await?
        {
            return Err("The provided version does not exists on this file".into());
        }

        // First get the file
        let file = self.get_file_by_id_owner(file_id, owner).await?;

        // Get its version path so we can replace the original

        let restore_version_path = format!("{}/{}.{}", file.id, version, file.extension_to_str());

        // Now we need to create a new version path for that old file
        let new_version = Utc::now().timestamp_millis();

        let new_file_version_path =
            format!("{}/{}.{}", file.id, new_version, file.extension_to_str());

        let internal_full_filename = &format!("{}.{}", file.id, file.extension_to_str());

        // We move from the original path to the new path, in the version side
        self.storage
            .move_file(internal_full_filename, &new_file_version_path)
            .await?;

        // We can now move the file at the restore path, to the current path
        self.storage
            .move_file(&restore_version_path, internal_full_filename)
            .await?;

        // The previous file the we just moved in, is itself a version
        // Create a new version model with the versioning database
        self.version_db
            .create_version_with_file_id(FileVersion::new(&file, new_version, None))
            .await?;

        // Delete the old backup
        self.version_db
            .delete_version_by_file_id_version(file_id, version)
            .await?;

        let file = self.file_db.update_file_time(file_id).await?;
        Ok(file)
    }

    pub async fn delete_file_by_id(&self, file_id: &ObjectId) -> Result<()> {
        let deleted_file = self.file_db.delete_file_by_id(file_id).await?;

        let internal_full_filename =
            &format!("{}.{}", deleted_file.id, deleted_file.extension_to_str());
        let internal_file_version_path = &format!("{}/", deleted_file.id);

        self.storage.delete_file(internal_full_filename).await?;
        self.storage
            .delete_folder(internal_file_version_path)
            .await?;

        self.version_db.delete_versions_by_file_id(file_id).await?;
        Ok(())
    }
}
