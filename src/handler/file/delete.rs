use salvo::{handler, Depot, Request, Response};

use crate::{
    helper::{cookie::get_cookie_user_id, depot::get_file_service, param::get_param_file_id},
    web::Web,
    WebResult,
};

#[handler]
pub async fn delete_file_handler(
    req: &mut Request,
    depot: &Depot,
    res: &mut Response,
) -> WebResult {
    // First we need to get the file from param, and then check the owner
    // Then compare it against the logged in user
    // If they match then we can delete the file
    // But first, we have to check if the user logged in or not

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
