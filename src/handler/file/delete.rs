use salvo::{handler, Depot, Request, Response};

use crate::{
    helper::{
        cookie::get_cookie_user_id,
        depot::{get_file_service, get_param_file},
    },
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

    // Get the file from the storage
    let param_file = get_param_file(depot)?;

    // Else just delete the file
    file_service.delete_file_by_id(&param_file.id).await?;

    Ok(Web::ok("File deleted", ()))
}
