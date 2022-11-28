use salvo::{handler, Depot, Request, Response};

use crate::{
    helper::{
        cookie::get_cookie_user_id,
        depot::{get_file_service, get_file_version_service, get_param_file, get_user_service},
        param::get_param_version_number,
    },
    response::FinalFileResponse,
    web::Web,
    WebResult,
};

#[handler]
pub async fn delete_file_version_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    // Check if the user is logged in or not
    let cookie_user_id = get_cookie_user_id(depot)?;

    // Get the version number
    let version_number = get_param_version_number(req)?;

    let param_file = get_param_file(depot)?;

    let file_service = get_file_service(depot)?;
    let file_version_service = get_file_version_service(depot)?;

    // After that check is complete
    // Delete the version
    file_version_service
        .delete_version_by_file_version(param_file, version_number)
        .await?;

    // Refresh the result
    let param_file_id = param_file.id;

    Ok(Web::ok(
        "Delete version successfully",
        FinalFileResponse::new(
            param_file.clone(),
            get_user_service(depot)?
                .get_user_by_id(cookie_user_id)
                .await?,
            file_version_service
                .get_versions_by_file_id(&param_file_id)
                .await?,
        )?,
    ))
}
