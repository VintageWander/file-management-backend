use mongodb::bson::oid::ObjectId;

use crate::{
    aws::S3,
    base::{file::File, file_version::FileVersion},
    db::file_version_db::FileVersionDB,
    Result,
};

#[derive(Debug, Clone)]
pub struct FileVersionService {
    file_version_db: FileVersionDB,
    storage: S3,
}

impl FileVersionService {
    pub fn init(file_version_db: &FileVersionDB, s3: &S3) -> Self {
        Self {
            file_version_db: file_version_db.clone(),
            storage: s3.clone(),
        }
    }

    pub async fn get_versions_by_file_id(&self, file_id: &ObjectId) -> Result<Vec<FileVersion>> {
        self.file_version_db.get_versions_by_file_id(file_id).await
    }

    pub async fn get_version_by_file_id_version(
        &self,
        file_id: &ObjectId,
        version: i64,
    ) -> Result<FileVersion> {
        self.file_version_db
            .get_version_by_file_id_version(file_id, version)
            .await
    }

    pub async fn delete_version_by_file_version(&self, file: &File, version: i64) -> Result<()> {
        let file_id = file.id;
        let internal_file_version_path =
            &format!("{}/{}.{}", file.id, version, file.extension_to_str());
        self.file_version_db
            .delete_version_by_file_id_version(&file_id, version)
            .await?;
        self.storage.delete_file(internal_file_version_path).await?;
        Ok(())
    }
}
