use salvo::{handler, Depot, Request, Response};

use crate::{
    helper::{
        cookie::get_cookie_user_id_option,
        depot::{get_folder_service, get_user_service},
        param::get_param_folder_id,
    },
    response::FinalFolderResponse,
    web::Web,
    WebResult,
};

#[handler]
pub async fn get_public_folders_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
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
        responses
            .push(
                FinalFolderResponse::new(
                    folder, 
                    get_user_service(depot)?
                        .get_user_by_id(&owner)
                        .await?
                )?
            )
    }
    Ok(Web::ok("Get public folders successfully", responses))
}

#[handler]
pub async fn get_folder_by_id_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    let param_folder_id = get_param_folder_id(req)?;

    let folder_service = get_folder_service(depot)?;

    // Checks if the user is logged in or not
    let Some(cookie_user_id) = get_cookie_user_id_option(depot) else {
        let folder = folder_service
            .get_public_folder_by_id(&param_folder_id)
            .await?;
        let owner = folder.owner;
        return Ok(Web::ok("Get file by param success", 
        FinalFolderResponse::new(folder, 
            get_user_service(depot)?.get_user_by_id(&owner).await?
        )?
    ));
    };

    // If the user is logged in, and it matches with the owner of the file, return that folder
    // Else, still return that folder but only if it is public
    let folder = folder_service.get_folder_by_id(&param_folder_id).await?;
    let folder = match *cookie_user_id == folder.owner {
        true => folder,
        false => {
            folder_service
                .get_public_folder_by_id(&param_folder_id)
                .await?
        }
    };

    Ok(Web::ok(
        "Get file by param success",
        FinalFolderResponse::new(
            folder,
            get_user_service(depot)?
                .get_user_by_id(cookie_user_id)
                .await?,
        )?,
    ))
}
