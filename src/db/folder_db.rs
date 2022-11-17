use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::{bson::Document, Collection};

use crate::base::folder::Folder;
use crate::Result;

use super::mongo::DB;

#[derive(Debug, Clone)]
pub struct FolderDB {
    collection: Collection<Folder>,
}

impl FolderDB {
    pub fn init(db: &DB) -> Self {
        Self {
            collection: db.get_collection("Folder"),
        }
    }

    async fn get_folders_by(&self, doc: Document) -> Result<Vec<Folder>> {
        let folders = self.collection.find(doc, None).await?.try_collect().await?;
        Ok(folders)
    }

    pub async fn get_folders_by_owner(&self, owner: &ObjectId) -> Result<Vec<Folder>> {
        self.get_folders_by(doc! {"owner": owner}).await
    }

    pub async fn get_public_folders(&self) -> Result<Vec<Folder>> {
        self.get_folders_by(doc! {"visibility": "public"}).await
    }

    pub async fn get_public_folders_by_owner(&self, owner: &ObjectId) -> Result<Vec<Folder>> {
        self.get_folders_by(doc! {"owner": owner, "visibility": "public"})
            .await
    }

    async fn get_folder_by(&self, doc: Document) -> Result<Folder> {
        let folder = self
            .collection
            .find_one(doc, None)
            .await?
            .ok_or("Cannot find the folder with the provided information")?;
        Ok(folder)
    }

    pub async fn get_folder_by_id(&self, id: &ObjectId) -> Result<Folder> {
        self.get_folder_by(doc! {"_id": id}).await
    }

    pub async fn get_public_folder_by_id(&self, id: &ObjectId) -> Result<Folder> {
        self.get_folder_by(doc! {"_id": id, "visibility": "public"})
            .await
    }

    pub async fn get_folder_by_fullpath(&self, fullpath: &str) -> Result<Folder> {
        self.get_folder_by(doc! {"fullpath": fullpath}).await
    }

    pub async fn get_folders_by_prefix_position(&self, prefix: &str) -> Result<Vec<Folder>> {
        let position_regex = format!("/^{prefix}/");
        self.get_folders_by(doc! {
            "position": {
                "$regex": position_regex
            }
        })
        .await
    }

    async fn exists_folder_by(&self, doc: Document) -> Result<bool> {
        let count = self.collection.count_documents(doc, None).await?;
        Ok(count != 0)
    }

    pub async fn exists_folder_by_id(&self, id: &ObjectId) -> Result<bool> {
        self.exists_folder_by(doc! {"_id": id}).await
    }

    pub async fn exists_folder_by_fullpath(&self, fullpath: &str) -> Result<bool> {
        self.exists_folder_by(doc! {"fullpath": fullpath}).await
    }

    pub async fn exists_folder_by_position(&self, position: &str) -> Result<bool> {
        self.exists_folder_by(doc! {"position": position}).await
    }

    pub async fn create_folder(&self, folder: Folder) -> Result<Folder> {
        let id = self
            .collection
            .insert_one(folder, None)
            .await?
            .inserted_id
            .as_object_id()
            .ok_or("Cannot create a new folder")?;

        let folder = self.get_folder_by_id(&id).await?;

        Ok(folder)
    }

    pub async fn update_folder(&self, id: &ObjectId, folder: Folder) -> Result<Folder> {
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();
        let folder_doc: Document = folder.into();
        let folder = self
            .collection
            .find_one_and_update(doc! {"_id": id}, doc! {"$set": folder_doc}, options)
            .await?
            .ok_or("Cannot update the folder")?;

        Ok(folder)
    }

    pub async fn move_inner_folders(
        &self,
        old_position: &str,
        old_fullpath: &str,
        new_position: &str,
    ) -> Result<()> {
        // fullpath for regex search
        // use new position to replace old position

        let position_regex = mongodb::bson::Regex {
            pattern: format!("^{old_fullpath}"),
            options: String::new(),
        };

        self.collection
            .update_many(
                doc! {
                    "position": {
                        "$regex": position_regex.clone()
                    },
                    "fullpath": {
                        "$regex": position_regex
                    }
                },
                vec![
                    doc! {
                    "$set": {
                        "position": {
                            "$replaceOne": {
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
                                "$replaceOne": {
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

    pub async fn delete_folder_by_id_owner(
        &self,
        id: &ObjectId,
        owner: &ObjectId,
    ) -> Result<Folder> {
        let deleted_folder = self
            .collection
            .find_one_and_delete(doc! {"_id": id, "owner": owner}, None)
            .await?
            .ok_or("Cannot delete the folder")?;
        Ok(deleted_folder)
    }

    pub async fn delete_folders_by_owner(&self, owner: &ObjectId) -> Result<()> {
        self.collection
            .delete_many(doc! {"owner": owner}, None)
            .await?;
        Ok(())
    }

    pub async fn delete_folders_by_prefix_position(&self, prefix: &str) -> Result<()> {
        let position_regex = format!("/^{prefix}/");
        self.collection
            .delete_many(doc! {"position": {"$regex": position_regex}}, None)
            .await?;
        Ok(())
    }

    pub async fn delete_folders_by_prefix_fullpath(&self, prefix: &str) -> Result<()> {
        let fullpath_regex = format!("/^{prefix}/");
        self.collection
            .delete_many(doc! {"fullpath": {"$regex": fullpath_regex}}, None)
            .await?;
        Ok(())
    }
}
