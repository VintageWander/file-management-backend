use salvo::{handler, Depot, Request};

use crate::{
    error::Error,
    helper::{
        cookie::get_cookie_user_id,
        depot::{get_folder_service, get_param_folder},
        param::get_param_folder_id,
    },
    web::Web,
    WebResult,
};

#[handler]
pub async fn delete_folder_handler(req: &mut Request, depot: &Depot) -> WebResult {
    // First we need to get the folder from param, and then check the owner
    // Then compare it against the logged in user
    // If they match then we can delete the folder
    // But first, we have to check if the user logged in or not

    let cookie_user_id = get_cookie_user_id(depot)?;

    // Get the folder storage
    let folder_service = get_folder_service(depot)?;

    // Get the param id of the folder
    let param_folder_id = get_param_folder_id(req)?;

    // Get the folder from the storage
    let param_folder = get_param_folder(depot)?;

    // If the owners doesn't match
    if *cookie_user_id != param_folder.owner {
        // Return error
        return Err(Error::Permissions(
            "You cannot delete other user's folder".into(),
        ));
    }

    // Else just delete the folder
    folder_service
        .delete_folder_by_id_owner(&param_folder_id, cookie_user_id)
        .await?;

    Ok(Web::ok("Folder deleted", ()))
}
