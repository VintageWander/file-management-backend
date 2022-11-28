use salvo::{handler, Depot, Request, Response};

use crate::{
    helper::{
        body::extract_from_body,
        cookie::get_cookie_user_id,
        depot::{get_file_service, get_file_version_service, get_user_service},
    },
    request::file::restore::RestoreFileRequest,
    response::FinalFileResponse,
    web::Web,
    WebResult,
};

#[handler]
pub async fn restore_file_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    // Check if the user is logged in or not
    let cookie_user_id = get_cookie_user_id(depot)?;

    // Extract the restore file request
    let restore_req = extract_from_body::<RestoreFileRequest>(req).await?;

    // // First we get the param file id
    // let param_file_id = get_param_file_id(req)?;

    // // Then we get the version number
    // let version_number = get_version_number(req)?;

    // We get the file service
    let file_service = get_file_service(depot)?;

    let file = file_service
        .restore_file_from_version(&restore_req.file_id, cookie_user_id, restore_req.version)
        .await?;
    let file_id = file.id;

    Ok(Web::ok(
        "Success",
        FinalFileResponse::new(
            file,
            get_user_service(depot)?
                .get_user_by_id(cookie_user_id)
                .await?,
            get_file_version_service(depot)?
                .get_versions_by_file_id(&file_id)
                .await?,
        )?,
    ))
}
