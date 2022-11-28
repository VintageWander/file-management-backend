use salvo::{handler, Depot};

use crate::{
    helper::{
        cookie::get_cookie_user_id,
        depot::{get_file_service, get_folder_service, get_user_service},
    },
    response::FinalUserResponse,
    web::Web,
    WebResult,
};

#[handler]
pub async fn profile_handler(depot: &Depot) -> WebResult {
    // Get the cookie user id
    let cookie_user_id = get_cookie_user_id(depot)?;
    // Get the user storage
    let user_service = get_user_service(depot)?;

    // Find the user with the id inside the cookie
    let user = user_service.get_user_by_id(cookie_user_id).await?;

    let files = get_file_service(depot)?
        .get_files_by_owner(cookie_user_id)
        .await?;
    let folders = get_folder_service(depot)?
        .get_folders_by_owner(cookie_user_id)
        .await?;

    Ok(Web::ok(
        "Profile info successfully retrieved",
        FinalUserResponse::new(user, files, folders)?,
    ))
}
