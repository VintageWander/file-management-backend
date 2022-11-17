use salvo::Request;
use serde::Deserialize;

use crate::Result;

pub async fn extract_from_form<T: for<'a> Deserialize<'a>>(
    req: &mut Request,
) -> Result<T> {
    Ok(req.parse_form::<T>().await?)
}
