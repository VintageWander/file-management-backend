use salvo::Request;
use serde::Deserialize;

use crate::{error::Error, Result};

pub async fn extract_from_body<'a, T: Deserialize<'a>>(req: &'a mut Request) -> Result<T> {
    req.parse_body::<T>().await.map_err(Error::HttpParse)
}
