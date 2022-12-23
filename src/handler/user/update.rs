use futures::try_join;
use salvo::{handler, Depot, Request};

use crate::{
    error::Error,
    helper::{
        body::extract_from_body,
        cookie::get_cookie_user_id,
        depot::{get_file_service, get_folder_service, get_user_service},
        param::get_param_user_id,
    },
    request::user::update::UpdateUserRequest,
    response::FinalUserResponse,
    web::Web,
    WebResult,
};

#[handler]
pub async fn update_user_handler(req: &mut Request, depot: &Depot) -> WebResult {
    // Extract the update user request
    let user_req: UpdateUserRequest = extract_from_body(req).await?;

    // Get the user_db from depot
    let user_service = get_user_service(depot)?;

    // Get the param user id
    let param_user_id = get_param_user_id(req)?;

    // Find the param user
    let param_user = user_service.get_user_by_id(&param_user_id).await?;

    // Get the logged in user id (cookie user id)
    let cookie_user_id = get_cookie_user_id(depot)?;

    if param_user_id != *cookie_user_id {
        return Err(Error::Permissions(
            "You cannot update other user's profile".into(),
        ));
    }

    // Construct the user to update
    let update_user = user_req.into_user(param_user)?;

    // Update the user
    let changed_user = user_service.update_user(update_user).await?;

    // Get the files and folders that the user owns
    let (files, folders) = try_join!(
        get_file_service(depot)?.get_files_by_owner(cookie_user_id),
        get_folder_service(depot)?.get_folders_by_owner(cookie_user_id)
    )?;

    Ok(Web::ok(
        "Update user successfully",
        FinalUserResponse::new(changed_user, files, folders)?,
    ))
}
