use super::S3;
use crate::{
    helper::{into_string, position::get_folder_position},
    validation::file::{check_dir, check_fullpath},
    Result,
};

impl S3 {
    pub async fn rename_file(
        &self,
        fullpath: &str,
        rename_path: &str,
    ) -> Result<()> {
        check_fullpath(fullpath).map_err(into_string)?;
        check_fullpath(rename_path).map_err(into_string)?;

        if get_folder_position(fullpath)? != get_folder_position(rename_path)? {
            return Err(
                "This isn't a rename, use the move file function if you want to move".into(),
            );
        }

        if fullpath == rename_path {
            return Ok(());
        }
        self.move_file(fullpath, rename_path).await
    }

    pub async fn rename_folder(
        &self,
        dir_path: &str,
        rename_path: &str,
    ) -> Result<()> {
        check_dir(dir_path).map_err(into_string)?;
        check_dir(rename_path).map_err(into_string)?;

        if get_folder_position(dir_path)? != get_folder_position(rename_path)? {
            return Err(
                "This isn't a rename, use the move folder function if you want to move".into(),
            );
        }

        if dir_path == rename_path {
            return Ok(());
        }
        self.move_folder(dir_path, rename_path).await
    }
}
