use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::validation::user::{check_password, check_username};
use crate::{base::user::User, Result};

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    #[validate(custom = "check_username")]
    pub username: Option<String>,

    #[validate(email(message = "The email must be in correct form"))]
    pub email: Option<String>,

    #[validate(custom = "check_password")]
    pub password: String,

    #[validate(custom = "check_password")]
    pub new_password: Option<String>,

    #[validate(must_match(
        other = "new_password",
        message = "The password must match with confirmPassword"
    ))]
    pub confirm_password: Option<String>,
}

impl UpdateUserRequest {
    pub fn into_user(self, old_user: User) -> Result<User> {
        self.validate()?;
        if self.new_password != self.confirm_password {
            return Err("The new password and the confirm new password does not match".into());
        }
        if *old_user.password != self.password {
            return Err("The provided password does not match with the current password".into());
        }
        User::new(
            old_user.id,
            &self.username.unwrap_or(old_user.username),
            &self.email.unwrap_or(old_user.email),
            &self.new_password.unwrap_or(old_user.password),
            &old_user.refresh_token,
            Some(old_user.created_at),
        )
    }
}
