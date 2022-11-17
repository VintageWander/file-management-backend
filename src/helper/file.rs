use crate::Result;
use salvo::{http::form::FilePart, Request};

pub async fn get_file_from_req<'a>(
    req: &'_ mut Request,
) -> Result<&'_ FilePart> {
    let file = req
        .file("file")
        .await
        .ok_or("Cannot get the file from request")?;
    Ok(file)
}

pub async fn get_file_from_req_option<'a>(
    req: &'_ mut Request,
) -> Option<&'_ FilePart> {
    req.file("file").await
}
