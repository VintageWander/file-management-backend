use salvo::{handler, Depot, Request, Response};

use crate::{
    helper::{
        body::extract_from_body,
        cookie::get_cookie_user_id,
        depot::{get_folder_service, get_user_service},
    },
    request::folder::create::CreateFolderRequest,
    web::Web,
    WebResult,
};

#[handler]
pub async fn create_folder_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    // Checks if the user has logged in or not
    let cookie_user_id = get_cookie_user_id(depot)?;

    // Extract the data from request
    let folder_req = extract_from_body::<CreateFolderRequest>(req).await?;

    // Get the file service
    let folder_service = get_folder_service(depot)?;

    // Get the cookie user
    let cookie_user = get_user_service(depot)?
        .get_user_by_id(cookie_user_id)
        .await?;

    let folder_model = folder_req.into_folder(&cookie_user)?;

    let created_folder = folder_service
        .create_folder(folder_model)
        .await?
        .into_response()?;

    // Return back the created file
    Ok(Web::ok("Create file successfully", created_folder))
}
