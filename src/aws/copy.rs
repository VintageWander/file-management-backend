use super::S3;
use crate::{
    helper::into_string,
    validation::file::{check_dir, check_fullpath},
    Result,
};

impl S3 {
    pub async fn copy_file(
        &self,
        fullpath: &str,
        dest_fullpath: &str,
    ) -> Result<()> {
        check_fullpath(fullpath).map_err(into_string)?;
        check_fullpath(dest_fullpath).map_err(into_string)?;

        let src = format!("{}/{fullpath}", self.bucket_name);

        self.client
            .copy_object()
            .copy_source(src)
            .bucket(&self.bucket_name)
            .key(dest_fullpath)
            .send()
            .await?;

        Ok(())
    }

    pub async fn copy_folder(&self, dir: &str, dest_dir: &str) -> Result<()> {
        check_dir(dir).map_err(into_string)?;
        check_dir(dest_dir).map_err(into_string)?;

        let objs = self.get_all(dir).await?;

        for obj in objs {
            let src = format!("{}/{obj}", self.bucket_name);
            let dest = format!("{dest_dir}{}", obj.split_at(dir.len()).1);
            self.client
                .copy_object()
                .copy_source(src)
                .bucket(&self.bucket_name)
                .key(dest)
                .send()
                .await?;
        }

        Ok(())
    }
}
