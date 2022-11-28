use salvo::{handler, Depot, Request};

use crate::{
    helper::{body::extract_from_body, cookie::get_cookie_user, depot::get_folder_service},
    request::folder::create::CreateFolderRequest,
    web::Web,
    WebResult,
};

#[handler]
pub async fn create_folder_handler(req: &mut Request, depot: &mut Depot) -> WebResult {
    // Checks if the user has logged in or not

    // Extract the data from request
    let folder_req = extract_from_body::<CreateFolderRequest>(req).await?;

    // Get the file service
    let folder_service = get_folder_service(depot)?;

    // Get the cookie user
    let cookie_user = get_cookie_user(depot)?;

    let folder_model = folder_req.into_folder(cookie_user)?;

    let created_folder = folder_service
        .create_folder(folder_model)
        .await?
        .into_response()?;

    // Return back the created file
    Ok(Web::ok("Create file successfully", created_folder))
}
