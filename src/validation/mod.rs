use fancy_regex::Regex;
use validator::ValidationError;

use crate::helper::make_error::validation_message;

pub mod aws;
pub mod file;
pub mod user;

pub fn check_with(
    test_str: &str,
    regex_str: &str,
    fail_message: &'static str,
) -> Result<(), ValidationError> {
    let regex = Regex::new(regex_str)
        .map_err(|_| validation_message("Invalid Regex"))?;
    let result = regex
        .is_match(test_str)
        .map_err(|_| validation_message("Matching process failed"))?;

    match result {
        true => Ok(()),
        false => Err(validation_message(fail_message)),
    }
}
