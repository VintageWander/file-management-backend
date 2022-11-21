use salvo::Request;
use serde::Deserialize;

use crate::Result;

pub async fn extract_from_body<'a, T: Deserialize<'a>>(req: &'a mut Request) -> Result<T> {
    Ok(req.parse_body::<T>().await?)
}
