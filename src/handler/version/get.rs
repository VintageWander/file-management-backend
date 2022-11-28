use salvo::{handler, Depot, Request, Response};
use serde_json::json;

use crate::{
    helper::{
        cookie::get_cookie_user_id,
        depot::{get_file_version_service, get_param_file},
        param::get_param_version_number,
    },
    web::Web,
    WebResult,
};

#[handler]
pub async fn get_versions_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    let cookie_user_id = get_cookie_user_id(depot)?;

    let param_file = get_param_file(depot)?;

    let versions = get_file_version_service(depot)?
        .get_versions_by_file_id(&param_file.id)
        .await?
        .into_iter()
        .map(|v| v.version_number)
        .collect::<Vec<_>>();

    Ok(Web::ok(
        "Get versions from file success",
        json!({
            "versions": versions,
        }),
    ))
}

#[handler]
pub async fn get_version_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    let cookie_user_id = get_cookie_user_id(depot)?;

    let param_file_id = get_param_file(depot)?.id;

    let version_number = get_param_version_number(req)?;

    let file_version_service = get_file_version_service(depot)?;

    let version = file_version_service
        .get_version_by_file_id_version(&param_file_id, version_number)
        .await?;

    Ok(Web::ok(
        "Get version successfully",
        json!({
            "version": version.version_number
        }),
    ))
}
