use aws_sdk_s3::types::ByteStream;
use aws_smithy_http::body::SdkBody;

use super::S3;
use crate::{
    helper::into_string,
    validation::file::{check_dir, check_fullpath},
    Result,
};

impl S3 {
    pub async fn create_file(
        &self,
        fullpath: &str,
        data: Vec<u8>,
    ) -> Result<()> {
        check_fullpath(fullpath).map_err(into_string)?;

        let mime = mime_guess::from_path(fullpath)
            .first_or_octet_stream()
            .to_string();
        let body = ByteStream::from(data);

        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(fullpath)
            .body(body)
            .content_type(&mime)
            .send()
            .await?;

        Ok(())
    }

    pub async fn create_folder(&self, fullpath: &str) -> Result<()> {
        check_dir(fullpath).map_err(into_string)?;

        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(fullpath)
            .body(ByteStream::from(SdkBody::empty()))
            .send()
            .await?;
        Ok(())
    }
}
