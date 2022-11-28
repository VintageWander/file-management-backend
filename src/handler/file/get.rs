use salvo::{handler, Depot};

use crate::{
    helper::depot::{get_file_service, get_file_version_service, get_param_file, get_user_service},
    response::FinalFileResponse,
    web::Web,
    WebResult,
};

#[handler]
pub async fn get_public_files_handler(depot: &mut Depot) -> WebResult {
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
pub async fn get_file_by_id_handler(depot: &mut Depot) -> WebResult {
    // Get the param file
    let param_file = get_param_file(depot)?;

    // Get the user service
    let user_service = get_user_service(depot)?;

    let owner = user_service.get_user_by_id(&param_file.owner).await?;
    let versions = get_file_version_service(depot)?
        .get_versions_by_file_id(&param_file.id)
        .await?;

    Ok(Web::ok(
        "Get file by param success",
        FinalFileResponse::new(param_file.clone(), owner, versions)?,
    ))
}
