use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::base::file::File;
use crate::base::user::User;
use crate::validation::user::check_username;
use crate::Result;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct UserResponse {
    pub id: String,
    #[validate(custom = "check_username")]
    pub username: String,

    #[validate(email(message = "The email must be in correct form"))]
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id.to_string(),
            username: u.username,
            email: u.email,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

impl UserResponse {
    pub fn from_user(u: User) -> Result<Self> {
        let res = Self {
            id: u.id.to_string(),
            username: u.username,
            email: u.email,
            created_at: u.created_at,
            updated_at: u.updated_at,
        };

        res.validate()?;
        Ok(res)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserWithFileResponse {
    pub id: String,

    #[validate(custom = "check_username")]
    pub username: String,
    #[validate(email(message = "The email must be in correct form"))]
    pub email: String,

    pub files: Vec<File>,

    pub created_at: i64,
    pub updated_at: i64,
}
