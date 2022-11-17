use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    base::user::User,
    validation::user::{check_password, check_username},
    Result,
};

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    #[validate(custom = "check_username")]
    pub username: String,

    #[validate(email(message = "The email must be in correct form"))]
    pub email: String,

    #[validate(custom = "check_password")]
    pub password: String,

    #[validate(must_match(
        other = "password",
        message = "The password must match with confirmPassword"
    ))]
    pub confirm_password: String,
}

impl CreateUserRequest {
    pub fn into_user(self) -> Result<User> {
        self.validate()?;
        User::new(
            ObjectId::new(),
            &self.username,
            &self.email,
            &self.password,
            "",
            None,
        )
    }
}
