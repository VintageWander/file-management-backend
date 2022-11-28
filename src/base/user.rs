use chrono::Utc;
use mongodb::bson::{doc, oid::ObjectId, Document};

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::validation::user::{check_password, check_username};
use crate::{response::user::UserResponse, Result};

#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    #[validate(custom = "check_username")]
    pub username: String,

    #[validate(email(message = "Email must be in correct form"))]
    pub email: String,

    #[validate(custom = "check_password")]
    pub password: String,

    pub refresh_token: String,

    pub created_at: i64,
    pub updated_at: i64,
}

impl From<User> for Document {
    fn from(u: User) -> Self {
        doc! {
            "username": u.username,
            "email": u.email,
            "password": u.password,
            "createdAt": u.created_at,
            "updatedAt": u.updated_at,
        }
    }
}

impl User {
    // I made a constructor, the difference from creating a struct directly is that
    // This validates input before finish creation
    pub fn new(
        id: ObjectId,
        username: &str,
        email: &str,
        password: &str,
        refresh_token: &str,
        created_at: Option<i64>,
    ) -> Result<Self> {
        let user = Self {
            id,
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            refresh_token: refresh_token.to_string(),
            created_at: created_at.unwrap_or_else(|| Utc::now().timestamp_millis()),
            updated_at: Utc::now().timestamp_millis(),
        };
        user.validate()?;
        Ok(user)
    }

    pub fn into_response(self) -> Result<UserResponse> {
        UserResponse::from_user(self)
    }
}
