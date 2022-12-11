use std::{collections::HashMap, str::FromStr};

use mongodb::bson::oid::ObjectId;
use salvo::{handler, Depot, Request};

use crate::{
    helper::{
        cookie::get_cookie_user_id_option,
        depot::{get_folder_service, get_param_folder, get_user_service},
        param::get_param_folder_id,
    },
    response::FinalFolderResponse,
    web::Web,
    WebResult,
};

#[handler]
pub async fn get_folders_handler(req: &mut Request, depot: &mut Depot) -> WebResult {
    // Get the query data
    let mut queries = req
        .queries()
        .iter()
        .map(|i| (i.0.clone(), i.1.clone()))
        .collect::<HashMap<_, _>>();

    let cookie_user_id_option = get_cookie_user_id_option(depot);

    match (cookie_user_id_option, queries.get(&"owner".to_string())) {
        (Some(cookie_user_id), Some(query_owner)) => {
            if *cookie_user_id != ObjectId::from_str(query_owner)? {
                queries.insert("visibility".to_string(), "public".to_string());
            }
        }
        _ => {
            queries.insert("visibility".to_string(), "public".to_string());
        }
    };

    let folders = get_folder_service(depot)?
        .get_folders_by_map(&queries)
        .await?;

    let mut responses = vec![];

    for folder in folders {
        let owner = get_user_service(depot)?
            .get_user_by_id(&folder.owner)
            .await?;
        responses.push(FinalFolderResponse::new(folder, owner)?)
    }

    Ok(Web::ok("Get folders successfully", responses))
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
