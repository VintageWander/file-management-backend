use salvo::{handler, Depot, Request, Response};

use crate::{
    helper::{
        cookie::get_cookie_user_id_option,
        depot::{get_file_service, get_file_version_service, get_user_service},
        param::get_param_file_id,
    },
    response::FinalFileResponse,
    web::Web,
    WebResult,
};

#[handler]
pub async fn get_public_files_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    let files = get_file_service(depot)?.get_public_files().await?;
    let mut responses = vec![];

    for file in files {
        let owner = get_user_service(depot)?.get_user_by_id(&file.owner).await?;
        let versions = get_file_version_service(depot)?
            .get_versions_by_file_id(&file.id)
            .await?;
        responses.push(FinalFileResponse::new(file, owner, versions)?)
    }

    Ok(Web::ok("Get public files successfully", responses))
}

#[handler]
pub async fn get_file_by_id_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    // Get the param file id
    let param_file_id = get_param_file_id(req)?;

    // Get the file service
    let file_service = get_file_service(depot)?;

    // Get the user service
    let user_service = get_user_service(depot)?;

    // Checks if the user is logged in or not
    let Some(cookie_user_id) = get_cookie_user_id_option(depot) else {
        let file = file_service
            .get_public_file_by_id(&param_file_id)
            .await?;
        let owner = user_service.get_user_by_id(&file.owner).await?;
        let versions = get_file_version_service(depot)?.get_versions_by_file_id(&file.id).await?;
        return Ok(Web::ok("Get file by param success", FinalFileResponse::new(file, owner, versions)?));
    };

    // If the user is logged in, and it matches with the owner of the file, return that file
    // Else, still return that file but only if it is public
    let file = file_service.get_file_by_id(&param_file_id).await?;
    let file = match *cookie_user_id == file.owner {
        true => file,
        false => file_service.get_public_file_by_id(&param_file_id).await?,
    };
    let owner = user_service.get_user_by_id(&file.owner).await?;
    let versions = get_file_version_service(depot)?
        .get_versions_by_file_id(&file.id)
        .await?;

    Ok(Web::ok(
        "Get file by param success",
        FinalFileResponse::new(file, owner, versions)?,
    ))
}
