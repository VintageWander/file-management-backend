use salvo::{handler, Depot, Request, Response};

use crate::{
    helper::{
        cookie::get_cookie_user_id,
        depot::get_file_service,
        param::{get_param_file_id, get_version_number},
    },
    web::Web,
    WebResult,
};

#[handler]
pub async fn restore_file_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    // First we get the param file id
    let param_file_id = get_param_file_id(req)?;

    // Then we get the version number
    let version_number = get_version_number(req)?;

    // We get the file service
    let file_service = get_file_service(depot)?;

    // Check if the user is logged in or not
    let cookie_user_id = get_cookie_user_id(depot)?;

    let file = file_service
        .restore_file_from_version(&param_file_id, cookie_user_id, version_number)
        .await?
        .into_response()?;

    Ok(Web::ok("Success", file))
}
