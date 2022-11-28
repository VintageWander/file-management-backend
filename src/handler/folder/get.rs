use salvo::{handler, Depot, Request};

use crate::{
    helper::{
        depot::{get_folder_service, get_param_folder, get_user_service},
        param::get_param_folder_id,
    },
    response::FinalFolderResponse,
    web::Web,
    WebResult,
};

#[handler]
pub async fn get_public_folders_handler(req: &mut Request, depot: &mut Depot) -> WebResult {
    let folders = match req.query::<&str>("position") {
        None => get_folder_service(depot)?.get_public_folders().await?,
        Some(query) => {
            get_folder_service(depot)?
                .get_public_folders_by_prefix_position(query)
                .await?
        }
    };
    let mut responses = vec![];
    for folder in folders {
        let owner = folder.owner;
        responses.push(FinalFolderResponse::new(
            folder,
            get_user_service(depot)?.get_user_by_id(&owner).await?,
        )?)
    }
    Ok(Web::ok("Get public folders successfully", responses))
}

#[handler]
pub async fn get_folder_by_id_handler(req: &mut Request, depot: &mut Depot) -> WebResult {
    let param_folder_id = get_param_folder_id(req)?;

    let folder_service = get_folder_service(depot)?;

    let param_folder = get_param_folder(depot)?;

    Ok(Web::ok(
        "Get file by param success",
        FinalFolderResponse::new(
            param_folder.clone(),
            get_user_service(depot)?
                .get_user_by_id(&param_folder.owner)
                .await?,
        )?,
    ))
}
