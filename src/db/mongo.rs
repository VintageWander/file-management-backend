use crate::Result;
use mongodb::{options::ClientOptions, Client, Collection};

#[derive(Debug, Clone)]
pub struct DB {
    client: Client,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mongodb_uri: String = std::env::var("MONGODB_URI")?;
        let mut client_options = ClientOptions::parse(mongodb_uri).await?;

        client_options.app_name = Some("file-manager-backend".to_string());
        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    pub fn get_collection<T>(&self, coll_name: &str) -> Collection<T> {
        self.client.database("final-db").collection(coll_name)
    }
}
