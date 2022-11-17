use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::validation::user::{check_password, check_username};
use crate::Result;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(custom = "check_username")]
    pub username: String,

    #[validate(custom = "check_password")]
    pub password: String,
}

impl LoginRequest {
    pub fn validate_self(self) -> Result<Self> {
        self.validate()?;
        Ok(self)
    }
}
