use std::cmp::min;

use crate::validation::file::check_dir;

use super::into_string;
use crate::Result;

pub fn get_common_path(path1: &str, path2: &str) -> Result<String> {
    check_dir(path1).map_err(into_string)?;
    check_dir(path2).map_err(into_string)?;

    let mut result = "".to_string();

    let length = min(path1.len(), path2.len());

    let mut iter1 = path1.chars();
    let mut iter2 = path2.chars();

    for _ in 0..length {
        let char1 = iter1.next().ok_or("Cannot get char 1")?;
        let char2 = iter2.next().ok_or("Cannot get char 2")?;
        if char1 != char2 {
            break;
        } else {
            result.push(char1);
        }
    }
    Ok(result)
}
