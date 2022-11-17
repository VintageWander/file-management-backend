use mongodb::bson::oid::ObjectId;
use salvo::{handler, Depot, Request};

use crate::helper::cookie::get_cookie_user_id_option;
use crate::helper::depot::{get_file_service, get_folder_service};
use crate::response::Response;
use crate::{
    helper::{depot::get_user_service, param::get_param_user_id},
    web::Web,
    WebResult,
};

#[handler]
pub async fn get_users_handler(req: &mut Request, depot: &mut Depot) -> WebResult {
    // Get the user_db from depot
    let user_service = get_user_service(depot)?;

    // Get the list of users
    let users = user_service.get_users().await?;

    let mut user_responses = vec![];
    for u in users {
        user_responses.push(u.into_response()?)
    }

    Ok(Web::ok("Get all users successfully", user_responses))
}

#[handler]
pub async fn get_user_handler(req: &mut Request, depot: &mut Depot) -> WebResult {
    // Get user_service from depot
    let user_service = get_user_service(depot)?;

    // Get the param user id from param
    let param_user_id: ObjectId = get_param_user_id(req)?;

    let file_service = get_file_service(depot)?;
    let folder_service = get_folder_service(depot)?;

    let (files, folders) = match get_cookie_user_id_option(depot) {
        Some(cookie_user_id) => {
            if *cookie_user_id != param_user_id {
                (
                    file_service
                        .get_public_files_by_owner(cookie_user_id)
                        .await?,
                    folder_service
                        .get_public_folders_by_owner(cookie_user_id)
                        .await?,
                )
            } else {
                (
                    file_service.get_files_by_owner(cookie_user_id).await?,
                    folder_service.get_folders_by_owner(cookie_user_id).await?,
                )
            }
        }
        None => (
            file_service
                .get_public_files_by_owner(&param_user_id)
                .await?,
            folder_service
                .get_public_folders_by_owner(&param_user_id)
                .await?,
        ),
    };

    Ok(Web::ok(
        "Get user by id successfully",
        Response::new(
            user_service.get_user_by_id(&param_user_id).await?,
            files,
            folders,
        )?,
    ))
}
