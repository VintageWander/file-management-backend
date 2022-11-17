use chrono::Utc;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::{bson::Document, Collection};

use crate::base::file::File;
use crate::Result;

use super::mongo::DB;

#[derive(Debug, Clone)]
pub struct FileDB {
    collection: Collection<File>,
}

impl FileDB {
    pub fn init(db: &DB) -> Self {
        Self {
            collection: db.get_collection("File"),
        }
    }

    async fn get_files_by(&self, doc: Document) -> Result<Vec<File>> {
        let files = self.collection.find(doc, None).await?.try_collect().await?;
        Ok(files)
    }

    pub async fn get_public_files(&self) -> Result<Vec<File>> {
        self.get_files_by(doc! {"visibility": "public"}).await
    }

    pub async fn get_files_by_owner(&self, owner: &ObjectId) -> Result<Vec<File>> {
        self.get_files_by(doc! {"owner": owner}).await
    }

    pub async fn get_public_files_by_owner(&self, owner: &ObjectId) -> Result<Vec<File>> {
        self.get_files_by(doc! {"owner": owner, "visibility": "public"})
            .await
    }

    async fn get_file_by(&self, doc: Document) -> Result<File> {
        let file = self
            .collection
            .find_one(doc, None)
            .await?
            .ok_or("Cannot find the file with the provided information")?;
        Ok(file)
    }

    pub async fn get_file_by_id(&self, id: &ObjectId) -> Result<File> {
        self.get_file_by(doc! {"_id": id}).await
    }

    pub async fn get_public_file_by_id(&self, id: &ObjectId) -> Result<File> {
        self.get_file_by(doc! {"_id": id, "visibility": "public"})
            .await
    }

    // This function exists because then if the user is trying to update a file that does not
    // belong to them, this will just throw an error
    pub async fn get_file_by_id_owner(&self, id: &ObjectId, owner: &ObjectId) -> Result<File> {
        self.get_file_by(doc! {"_id": id, "owner": owner}).await
    }

    pub async fn get_file_by_fullpath(&self, fullpath: &str) -> Result<File> {
        self.get_file_by(doc! {"fullpath": fullpath}).await
    }

    pub async fn get_file_by_prefix_position(&self, prefix: &str) -> Result<File> {
        let position_regex = format!("/^{prefix}/");
        self.get_file_by(doc! {
            "position": {"$regex": position_regex}
        })
        .await
    }

    async fn exists_file_by(&self, doc: Document) -> Result<bool> {
        let count = self.collection.count_documents(doc, None).await?;
        Ok(count != 0)
    }

    pub async fn exists_file_by_id(&self, id: &ObjectId) -> Result<bool> {
        self.exists_file_by(doc! {"_id": id}).await
    }

    pub async fn exists_file_by_fullpath(&self, fullpath: &str) -> Result<bool> {
        self.exists_file_by(doc! {"fullpath": fullpath}).await
    }

    pub async fn create_file(&self, file: File) -> Result<File> {
        let new_file_id = self
            .collection
            .insert_one(file, None)
            .await?
            .inserted_id
            .as_object_id()
            .ok_or("Cannot create a new file")?;
        let new_file = self.get_file_by_id(&new_file_id).await?;
        Ok(new_file)
    }

    pub async fn update_file_by_id(&self, id: &ObjectId, file: File) -> Result<File> {
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let file_doc: Document = file.into();

        let file = self
            .collection
            .find_one_and_update(doc! {"_id": id}, doc! {"$set": file_doc}, options)
            .await?
            .ok_or("Cannot update the file")?;

        Ok(file)
    }

    pub async fn update_file_time(&self, id: &ObjectId) -> Result<File> {
        // Update the file
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let file = self
            .collection
            .find_one_and_update(
                doc! {"_id": id},
                doc! {"$set":
                    { "updatedAt": Utc::now().timestamp_millis() }
                },
                options,
            )
            .await?
            .ok_or("Cannot update time")?;

        Ok(file)
    }

    pub async fn move_inner_files(
        &self,
        old_position: &str,
        old_fullpath: &str,
        new_position: &str,
        new_fullpath: &str,
    ) -> Result<()> {
        let position_regex = format!("/^{old_position}/");
        let fullpath_regex = format!("/^{old_fullpath}/");

        self.collection
            .update_many(
                doc! {
                    "position": {
                        "$regex": position_regex
                    },
                    "fullpath": {
                        "$regex": fullpath_regex
                    }
                },
                doc! {
                    "$set": {
                        "position": {
                            "$replaceOne": {
                                "input": "$position",
                                "find": old_position,
                                "replacement": new_position
                            }
                        },
                        "fullpath": {
                            "$replaceOne": {
                                "input": "$fullpath",
                                "find": old_fullpath,
                                "replacement": new_fullpath
                            }
                        }
                    }
                },
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn delete_file_by_id(&self, id: &ObjectId) -> Result<File> {
        let deleted_file = self
            .collection
            .find_one_and_delete(doc! {"_id": id}, None)
            .await?
            .ok_or("Cannot delete the file")?;
        Ok(deleted_file)
    }

    pub async fn delete_files_by_owner(&self, owner: &ObjectId) -> Result<()> {
        self.collection
            .delete_many(doc! {"owner": owner}, None)
            .await?;
        Ok(())
    }

    pub async fn delete_files_by_prefix_position(&self, prefix: &str) -> Result<()> {
        let position_regex = format!("/^{prefix}/");
        self.collection
            .delete_many(
                doc! {
                "position": {"$regex": position_regex}},
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn delete_files_by_prefix_fullpath(&self, prefix: &str) -> Result<()> {
        let fullpath_regex = format!("/^{prefix}/");
        self.collection
            .delete_many(
                doc! {
                "fullpath": {"$regex": fullpath_regex}},
                None,
            )
            .await?;
        Ok(())
    }
}
