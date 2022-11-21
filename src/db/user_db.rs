use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::Collection;

use crate::base::user::User;
use crate::Result;

use super::mongo::DB;

#[derive(Debug, Clone)]
pub struct UserDB {
    collection: Collection<User>,
}

impl UserDB {
    pub fn init(db: &DB) -> Self {
        Self {
            collection: db.get_collection("User"),
        }
    }

    async fn get_users_by(&self, doc: Document) -> Result<Vec<User>> {
        let users = self.collection.find(doc, None).await?.try_collect().await?;
        Ok(users)
    }

    pub async fn get_users(&self) -> Result<Vec<User>> {
        self.get_users_by(doc! {}).await
    }

    async fn get_user_by(&self, doc: Document) -> Result<User> {
        let user = self
            .collection
            .find_one(doc, None)
            .await?
            .ok_or("Cannot get the user with the provided information")?;
        Ok(user)
    }

    pub async fn get_user_by_id(&self, id: &ObjectId) -> Result<User> {
        self.get_user_by(doc! {"_id": id}).await
    }

    pub async fn get_user_by_login_info(&self, username: &str, password: &str) -> Result<User> {
        self.get_user_by(doc! {"username": username, "password": password})
            .await
    }

    async fn exists_user_by(&self, doc: Document) -> Result<bool> {
        let count = self.collection.count_documents(doc, None).await?;
        Ok(count != 0)
    }

    pub async fn exists_user_by_id(&self, id: &ObjectId) -> Result<bool> {
        self.exists_user_by(doc! {"_id": id}).await
    }

    pub async fn exists_user_by_username(&self, username: &str) -> Result<bool> {
        self.exists_user_by(doc! {"username": username}).await
    }

    pub async fn exists_user_by_login_info(&self, username: &str, password: &str) -> Result<bool> {
        self.exists_user_by(doc! {"username": username, "password": password})
            .await
    }

    pub async fn exists_user_by_email(&self, email: &str) -> Result<bool> {
        self.exists_user_by(doc! {"email": email}).await
    }

    pub async fn create_user(&self, user: User) -> Result<User> {
        let new_user_id = self
            .collection
            .insert_one(user, None)
            .await?
            .inserted_id
            .as_object_id()
            .ok_or("Cannot create a new user")?;
        let user = self.get_user_by_id(&new_user_id).await?;
        Ok(user)
    }

    pub async fn update_user(&self, id: &ObjectId, user: User) -> Result<User> {
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let user_doc: Document = user.into();

        let user = self
            .collection
            .find_one_and_update(doc! {"_id": id}, doc! {"$set": user_doc}, options)
            .await?
            .ok_or("Cannot update the user")?;
        Ok(user)
    }

    pub async fn update_refresh_token(&self, id: &ObjectId, refresh_token: &str) -> Result<()> {
        self.collection
            .update_one(
                doc! {"_id": id},
                doc! {"$set": { "refreshToken": refresh_token } },
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn delete_user(&self, id: &ObjectId) -> Result<User> {
        let deleted_user = self
            .collection
            .find_one_and_delete(doc! {"_id": id}, None)
            .await?
            .ok_or("Cannot delete the user")?;
        Ok(deleted_user)
    }
}
