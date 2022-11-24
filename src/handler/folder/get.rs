use salvo::{handler, Depot, Request, Response};

use crate::{
    helper::{
        cookie::get_cookie_user_id_option, depot::get_folder_service, param::get_param_folder_id,
    },
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
        responses.push(folder.into_response()?)
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
        let file = folder_service
            .get_public_folder_by_id(&param_folder_id)
            .await?;
        return Ok(Web::ok("Get file by param success", file));
    };

    // If the user is logged in, and it matches with the owner of the file, return that folder
    // Else, still return that folder but only if it is public
    let file = folder_service.get_folder_by_id(&param_folder_id).await?;
    let file = match *cookie_user_id == file.owner {
        true => file,
        false => {
            folder_service
                .get_public_folder_by_id(&param_folder_id)
                .await?
        }
    };

    Ok(Web::ok("Get file by param success", ()))
}
