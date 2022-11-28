use salvo::{handler, Depot, Request};

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
pub async fn restore_file_handler(req: &mut Request, depot: &mut Depot) -> WebResult {
    // Check if the user is logged in or not
    let cookie_user_id = get_cookie_user_id(depot)?;

    let param_file_id = get_param_file(depot)?.id;

    let version_number = get_param_version_number(req)?;

    // We get the file service
    let file_service = get_file_service(depot)?;

    let file = file_service
        .restore_file_from_version(&param_file_id, cookie_user_id, version_number)
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
