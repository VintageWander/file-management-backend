use salvo::{handler, Depot};

use crate::{
    error::Error,
    helper::{
        cookie::get_cookie_user_id,
        depot::{get_file_service, get_param_file},
    },
    web::Web,
    WebResult,
};

#[handler]
pub async fn delete_file_handler(depot: &Depot) -> WebResult {
    // Get the logged in user
    let cookie_user_id = get_cookie_user_id(depot)?;

    // Get the file storage
    let file_service = get_file_service(depot)?;

    // Get the file from the storage
    let param_file = get_param_file(depot)?;

    if *cookie_user_id != param_file.owner {
        return Err(Error::Permissions(
            "You cannot delete other user's file".into(),
        ));
    }

    // Else just delete the file
    file_service.delete_file_by_id(&param_file.id).await?;

    Ok(Web::ok("File deleted", ()))
}
