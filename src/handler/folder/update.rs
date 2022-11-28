use salvo::{handler, Depot, Request};

use crate::{
    error::Error,
    helper::{
        body::extract_from_body,
        cookie::get_cookie_user,
        depot::{get_folder_service, get_param_folder},
        param::get_param_folder_id,
    },
    request::folder::update::UpdateFolderRequest,
    web::Web,
    WebResult,
};

#[handler]
pub async fn update_folder_handler(
    req: &mut Request,
    depot: &mut Depot,
    // res: &mut Response,
) -> WebResult {
    // Checks if the user has logged in or not
    // let cookie_user_id = get_cookie_user_id(depot)?;

    // Extract the data from request
    let folder_req = extract_from_body::<UpdateFolderRequest>(req).await?;

    // Get the folder db
    let folder_service = get_folder_service(depot)?;

    // Get the folder id from param
    let param_folder_id = get_param_folder_id(req)?;

    // Get the old folder
    let old_folder = get_param_folder(depot)?;

    // Get the cookie user
    let cookie_user = get_cookie_user(depot)?;

    if cookie_user.id != old_folder.owner {
        return Err(Error::Permissions(
            "You cannot update other user's folder".into(),
        ));
    }

    let folder_model = folder_req.into_folder(cookie_user, old_folder.clone())?;

    let updated_folder = folder_service
        .update_folder_by_id(&param_folder_id, folder_model)
        .await?
        .into_response()?;

    Ok(Web::ok("Update folder successfully", updated_folder))
}
