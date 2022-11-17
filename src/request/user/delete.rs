use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::validation::user::check_password;
use crate::Result;

#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserRequest {
    #[validate(custom(function = "check_password"))]
    pub password: String,

    #[validate(must_match(
        other = "password",
        message = "The password must match with confirmPassword"
    ))]
    pub confirm_password: String,
}

impl DeleteUserRequest {
    pub fn validate_self(self, current_password: &String) -> Result<Self> {
        self.validate()?;
        if self.password != *current_password {
            return Err("The provided password does not match with the current password".into());
        }
        Ok(self)
    }
}
