use crate::validation::file::{check_dir, check_fullpath};
use crate::Result;

use super::into_string;

pub fn get_folder_position(str: &str) -> Result<String> {
    check_dir(str).map_err(into_string)?;
    let mut result = str.split('/').collect::<Vec<_>>();
    result.pop();
    result.pop();
    let mut result = result.join("/");
    if result.is_empty() {
        return Ok(result);
    }
    result += "/";
    Ok(result)
}

pub fn get_file_position(str: &str) -> Result<String> {
    check_fullpath(str).map_err(into_string)?;
    let mut result = str.split('/').collect::<Vec<_>>();
    result.pop();
    let mut result = result.join("/");
    result += "/";
    Ok(result)
}
