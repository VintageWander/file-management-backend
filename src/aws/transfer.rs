use crate::{
    helper::into_string,
    validation::file::{check_dir, check_fullpath},
    Result,
};

use super::S3;

impl S3 {
    pub async fn move_file(&self, fullpath: &str, dest_fullpath: &str) -> Result<()> {
        check_fullpath(fullpath).map_err(into_string)?;
        check_fullpath(dest_fullpath).map_err(into_string)?;

        self.copy_file(fullpath, dest_fullpath).await?;
        self.delete_file(fullpath).await?;

        Ok(())
    }

    pub async fn move_folder(&self, dir_path: &str, dest_dir_path: &str) -> Result<()> {
        check_dir(dir_path).map_err(into_string)?;
        check_dir(dest_dir_path).map_err(into_string)?;

        self.copy_folder(dir_path, dest_dir_path).await?;
        self.delete_folder(dir_path).await?;

        Ok(())
    }
}
