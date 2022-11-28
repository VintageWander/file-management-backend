use salvo::{handler, Depot, Request, Response};

use crate::{
    helper::{
        body::extract_from_body,
        cookie::get_cookie_user_id,
        depot::{get_file_service, get_file_version_service, get_user_service},
        param::get_param_file_id,
    },
    request::file::delete::DeleteFileVersionRequest,
    response::FinalFileResponse,
    web::Web,
    WebResult,
};

#[handler]
pub async fn delete_file_handler(
    req: &mut Request,
    depot: &Depot,
    res: &mut Response,
) -> WebResult {
    // First we need to get the file from param, and then get the owner
    // The function get_file_by_id_owner means that it'll search for a file with owner and file id also
    // This eliminates the need for checking if logged in user id != param file owner id

    let cookie_user_id = get_cookie_user_id(depot)?;

    // Get the file storage
    let file_service = get_file_service(depot)?;

    // Get the param id of the file
    let param_file_id = get_param_file_id(req)?;

    // Get the file from the storage
    let param_file = file_service
        .get_file_by_id_owner(&param_file_id, cookie_user_id)
        .await?;

    // Else just delete the file
    file_service.delete_file_by_id(&param_file_id).await?;

    Ok(Web::ok("File deleted", ()))
}

#[handler]
pub async fn delete_file_version_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    // Check if the user is logged in or not
    let cookie_user_id = get_cookie_user_id(depot)?;

    // Extract the request from body
    let delete_version_req = extract_from_body::<DeleteFileVersionRequest>(req).await?;

    let file_service = get_file_service(depot)?;
    let file_version_service = get_file_version_service(depot)?;

    // Get the file from request file id and the cookie user id
    // This will throw a "Not Found" error if other user is trying to delete our file version
    let file = file_service
        .get_file_by_id_owner(&delete_version_req.file_id, cookie_user_id)
        .await?;
    let file_id = file.id;

    // After that check is complete
    // Delete the version
    file_version_service
        .delete_version_by_file_version(&file, delete_version_req.version)
        .await?;

    // Refresh the result

    Ok(Web::ok(
        "Delete version successfully",
        FinalFileResponse::new(
            file,
            get_user_service(depot)?
                .get_user_by_id(cookie_user_id)
                .await?,
            file_version_service
                .get_versions_by_file_id(&file_id)
                .await?,
        )?,
    ))
}
