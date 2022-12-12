use chrono::Utc;
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Regex};
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

    pub async fn get_files_by(&self, doc: Document) -> Result<Vec<File>> {
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

    // This function is important because it is used for getting the file at id, but it has to be public
    // Useful for returning a public file for visitors
    // It will purposely fail if the file at id is private
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

    pub async fn get_files_by_prefix_position(&self, prefix: &str) -> Result<Vec<File>> {
        let position_regex = Regex {
            pattern: format!("^{prefix}"),
            options: String::new(),
        };
        self.get_files_by(doc! {
            "position": {"$regex": position_regex}
        })
        .await
    }

    pub async fn get_files_by_prefix_exact_position(&self, prefix: &str) -> Result<Vec<File>> {
        let position_regex = Regex {
            pattern: format!("^{prefix}$"),
            options: String::new(),
        };
        self.get_files_by(doc! {
            "position": {"$regex": position_regex}
        })
        .await
    }

    pub async fn get_files_by_prefix_fullpath(&self, prefix: &str) -> Result<Vec<File>> {
        let position_regex = Regex {
            pattern: format!("^{prefix}"),
            options: String::new(),
        };
        self.get_files_by(doc! {
            "fullpath": {"$regex": position_regex}
        })
        .await
    }

    pub async fn get_public_files_by_prefix_position(&self, prefix: &str) -> Result<Vec<File>> {
        let position_regex = Regex {
            pattern: format!("^{prefix}"),
            options: String::new(),
        };
        self.get_files_by(doc! {
            "visibility": "public",
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
    ) -> Result<()> {
        // fullpath for regex search
        // use new position to replace old position

        let position_regex = &Regex {
            pattern: format!("^{old_fullpath}"),
            options: String::new(),
        };

        self.collection
            .update_many(
                doc! {
                    "fullpath": {
                        "$regex": position_regex
                    }
                },
                vec![
                    doc! {
                        "$set": {
                            "position": {
                                "$replaceAll": {
                                    "input": "$position",
                                    "find": old_position,
                                    "replacement": new_position
                                }
                            }
                        }
                    },
                    doc! {
                        "$set": {
                            "fullpath": {
                                "$replaceAll": {
                                    "input": "$fullpath",
                                    "find": old_position,
                                    "replacement": new_position
                                }
                            }
                        }
                    },
                ],
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
        let position_regex = Regex {
            pattern: format!("^{prefix}"),
            options: String::new(),
        };
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
        let fullpath_regex = Regex {
            pattern: format!("^{prefix}"),
            options: String::new(),
        };
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
