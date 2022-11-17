pub mod copy;
pub mod create;
pub mod delete;
pub mod get;
pub mod rename;
pub mod transfer;

use aws_sdk_s3::{config, Client, Credentials, Region};

use crate::Result;

const S3_KEY_ID: &str = "S3_KEY_ID";
const S3_KEY_SECRET: &str = "S3_KEY_SECRET";
const REGION: &str = "REGION";
const BUCKET_NAME: &str = "BUCKET_NAME";

#[derive(Debug, Clone)]
pub struct S3 {
    client: Client,
    bucket_name: String,
}

impl S3 {
    pub fn init() -> Result<Self> {
        Ok(Self {
            client: Self::get_aws_client()?,
            bucket_name: std::env::var(BUCKET_NAME)?,
        })
    }

    fn get_aws_client() -> Result<Client> {
        // Get the id secret from env
        let key_id = std::env::var(S3_KEY_ID)?;
        let key_secret = std::env::var(S3_KEY_SECRET)?;
        let region_env = std::env::var(REGION)?;

        // Build the aws cred
        let cred = Credentials::new(key_id, key_secret, None, None, "get-from-env");

        // Build the aws config
        let region = Region::new(region_env);
        let conf_builder = config::Builder::new()
            .region(region)
            .credentials_provider(cred);
        let conf = conf_builder.build();

        // Build the aws client
        let client = Client::from_conf(conf);
        Ok(client)
    }
}
