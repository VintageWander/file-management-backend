use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection,
};

use crate::base::file_version::FileVersion;

use super::mongo::DB;
use crate::Result;

#[derive(Debug, Clone)]
pub struct FileVersionDB {
    collection: Collection<FileVersion>,
}

impl FileVersionDB {
    pub fn init(db: &DB) -> Self {
        Self {
            collection: db.get_collection("FileVersion"),
        }
    }

    pub async fn get_versions_by(&self, doc: Document) -> Result<Vec<FileVersion>> {
        let file_versions = self.collection.find(doc, None).await?.try_collect().await?;
        Ok(file_versions)
    }

    pub async fn get_versions_by_file_id(&self, file_id: &ObjectId) -> Result<Vec<FileVersion>> {
        self.get_versions_by(doc! {"file": file_id }).await
    }

    pub async fn get_version_by(&self, doc: Document) -> Result<FileVersion> {
        Ok(self
            .collection
            .find_one(doc, None)
            .await?
            .ok_or("Cannot find the version with the provided information")?)
    }

    pub async fn get_version_by_id(&self, id: &ObjectId) -> Result<FileVersion> {
        self.get_version_by(doc! {"_id": id}).await
    }

    pub async fn get_version_by_file_id_version(
        &self,
        file_id: &ObjectId,
        version: i64,
    ) -> Result<FileVersion> {
        self.get_version_by(doc! {"file": file_id, "versionNumber": version})
            .await
    }

    pub async fn exists_version_by(&self, doc: Document) -> Result<bool> {
        let count = self.collection.count_documents(doc, None).await?;
        Ok(count != 0)
    }

    pub async fn exists_version_by_id(&self, id: &ObjectId) -> Result<bool> {
        self.exists_version_by(doc! {"_id": id}).await
    }

    pub async fn exists_version_by_file_id_version(
        &self,
        file_id: &ObjectId,
        version: i64,
    ) -> Result<bool> {
        self.exists_version_by(doc! {"file": file_id, "versionNumber": version})
            .await
    }

    pub async fn create_version_with_file_id(&self, file_ver: FileVersion) -> Result<FileVersion> {
        let file_ver_id = self
            .collection
            .insert_one(file_ver, None)
            .await?
            .inserted_id
            .as_object_id()
            .ok_or("Cannot create a new version")?;
        let file_ver = self.get_version_by_id(&file_ver_id).await?;
        Ok(file_ver)
    }

    pub async fn delete_version_by_id(&self, id: &ObjectId) -> Result<()> {
        self.collection.delete_one(doc! {"_id": id}, None).await?;
        Ok(())
    }

    pub async fn delete_version_by_file_id_version(
        &self,
        file_id: &ObjectId,
        version: i64,
    ) -> Result<()> {
        self.collection
            .delete_one(doc! {"file": file_id, "versionNumber": version}, None)
            .await?;
        Ok(())
    }

    pub async fn delete_versions_by_file_id(&self, file_id: &ObjectId) -> Result<()> {
        self.collection
            .delete_many(doc! {"file": file_id}, None)
            .await?;
        Ok(())
    }
}
