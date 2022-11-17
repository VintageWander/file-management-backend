use super::file::check_dir;
use crate::helper::into_string;
use crate::Result;

pub fn check_prefix(prefix: &str) -> Result<()> {
    Ok(check_dir(prefix).map_err(into_string)?)
}
