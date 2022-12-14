use salvo::{handler, Depot, Request};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    error::Error,
    helper::{
        cookie::get_cookie_user,
        depot::{get_file_service, get_file_version_service, get_param_file},
        file::get_file_from_req_option,
        form::extract_from_form,
    },
    request::file::update::UpdateFileRequest,
    response::FinalFileResponse,
    web::Web,
    WebResult,
};

#[handler]
pub async fn update_file_handler(req: &mut Request, depot: &mut Depot) -> WebResult {
    // Checks if the user has logged in or not

    // Extract the data from request
    let file_req = extract_from_form::<UpdateFileRequest>(req).await?;

    // Get the file db
    let file_service = get_file_service(depot)?;

    // Get the old file
    let old_file = get_param_file(depot)?;

    // Get the cookie_user
    let cookie_user = get_cookie_user(depot)?;

    if cookie_user.id != old_file.owner {
        return Err(Error::Permissions(
            "You cannot update other user's file".into(),
        ));
    }

    // Get the attachment file
    // Since this is optional, I have deal with 2 cases
    // 1) The user actually uploads a new file to replace the old one
    // 2) The user doesn't upload anything
    let file_option = get_file_from_req_option(req).await;

    match file_option {
        // If there is a file
        Some(file) => {
            // Open the received file from temporary path
            let mut local_file = File::open(file.path()).await?;

            // Initialize the file stream
            let mut file_stream = vec![];

            // Read the file's byte stream and store it inside file_stream
            local_file.read_to_end(&mut file_stream).await?;

            // Get the filename
            let full_filename = file.name();

            // Construct the file model from request
            let file_model = file_req.into_file(full_filename, old_file.clone(), cookie_user)?;

            // Send the file model to the database
            let updated_file = file_service
                .update_file_by_id(&old_file.id, file_model, file_stream)
                .await?;
            let updated_file_id = updated_file.id;

            // Return back the created file
            Ok(Web::ok(
                "Update file successfully",
                FinalFileResponse::new(
                    updated_file,
                    cookie_user.clone(),
                    get_file_version_service(depot)?
                        .get_versions_by_file_id(&updated_file_id)
                        .await?,
                )?,
            ))
        }
        // If there is no file
        None => {
            // Construct a file model
            let file_model = file_req.into_file(None, old_file.clone(), cookie_user)?;
            // Send the information to the database
            // Without the file ( vec![] )
            let updated_file = file_service
                .update_file_by_id(&old_file.id, file_model, vec![])
                .await?;
            let updated_file_id = updated_file.id;

            Ok(Web::ok(
                "Update file successfully",
                FinalFileResponse::new(
                    updated_file,
                    cookie_user.clone(),
                    get_file_version_service(depot)?
                        .get_versions_by_file_id(&updated_file_id)
                        .await?,
                )?,
            ))
        }
    }
}
