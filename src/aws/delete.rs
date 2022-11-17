use aws_sdk_s3::model::{Delete, ObjectIdentifier};

use crate::{
    helper::into_string,
    validation::file::{check_dir, check_fullpath},
    Result,
};

use super::S3;

impl S3 {
    pub async fn delete_file(&self, fullpath: &str) -> Result<()> {
        check_fullpath(fullpath).map_err(into_string)?;
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(fullpath)
            .send()
            .await?;
        Ok(())
    }

    pub async fn delete_folder(&self, fullpath: &str) -> Result<()> {
        check_dir(fullpath).map_err(into_string)?;

        let objs = self.get_all(fullpath).await?;
        let delete = objs
            .into_iter()
            .map(|s| {
                ObjectIdentifier::builder()
                    .set_key(Some(s))
                    .build()
            })
            .collect::<Vec<_>>();

        let delete = Delete::builder()
            .set_objects(Some(delete))
            .build();

        self.client
            .delete_objects()
            .bucket(&self.bucket_name)
            .delete(delete)
            .send()
            .await?;

        Ok(())
    }
}
