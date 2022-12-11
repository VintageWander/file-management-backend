use std::{collections::HashMap, str::FromStr};

use mongodb::bson::oid::ObjectId;
use salvo::{handler, Depot, Request};

use crate::{
    helper::{
        cookie::get_cookie_user_id_option,
        depot::{get_file_service, get_file_version_service, get_param_file, get_user_service},
    },
    response::FinalFileResponse,
    web::Web,
    WebResult,
};

#[handler]
pub async fn get_files_handler(req: &mut Request, depot: &mut Depot) -> WebResult {
    // Get the query data
    let mut queries = req
        .queries()
        .iter()
        .map(|i| (i.0.clone(), i.1.clone()))
        .collect::<HashMap<_, _>>();

    let cookie_user_id_option = get_cookie_user_id_option(depot);

    let file_service = get_file_service(depot)?;
    let user_service = get_user_service(depot)?;

    let files = match (cookie_user_id_option, queries.get(&"owner".to_string())) {
        (Some(cookie_user_id), Some(query_owner)) => {
            let owner_id = ObjectId::from_str(query_owner)?;
            if *cookie_user_id == owner_id {
                file_service.get_files_by_map(&queries)
            } else {
                queries.insert("visibility".to_string(), "public".to_string());
                file_service.get_files_by_map(&queries)
            }
        }
        _ => {
            queries.insert("visibility".to_string(), "public".to_string());
            file_service.get_files_by_map(&queries)
        }
    };

    let files = files.await?;

    let mut responses = vec![];

    for file in files {
        let owner = user_service.get_user_by_id(&file.owner).await?;
        let versions = get_file_version_service(depot)?
            .get_versions_by_file_id(&file.id)
            .await?;
        responses.push(FinalFileResponse::new(file, owner, versions)?)
    }

    Ok(Web::ok("Get files successfully", responses))
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
