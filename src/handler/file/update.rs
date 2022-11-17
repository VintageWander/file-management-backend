use salvo::{handler, Depot, Request, Response};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    helper::{
        cookie::get_cookie_user_id, depot::get_file_service, file::get_file_from_req_option,
        form::extract_from_form, param::get_param_file_id,
    },
    request::file::update::UpdateFileRequest,
    web::Web,
    WebResult,
};

#[handler]
pub async fn update_file_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    // Checks if the user has logged in or not
    let cookie_user_id = get_cookie_user_id(depot)?;

    // Extract the data from request
    let file_req = extract_from_form::<UpdateFileRequest>(req).await?;

    // Get the file db
    let file_service = get_file_service(depot)?;

    // Get the file id from param
    let param_file_id = get_param_file_id(req)?;

    // Get the old file
    let old_file = file_service
        .get_file_by_id_owner(&param_file_id, cookie_user_id)
        .await?;

    // Get the attachment file (optionally)
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
            let file_model = file_req.into_file(full_filename, old_file)?;

            // Send the file model to the database
            let updated_file = file_service
                .update_file_by_id(&param_file_id, file_model, file_stream)
                .await?
                .into_response()?;

            // Return back the created file
            Ok(Web::ok("Update file successfully", updated_file))
        }
        None => {
            // If there is no file
            // Construct a file model
            let file_model = file_req.into_file(None, old_file)?;
            // Send the information to the database
            // Without the file ( vec![] )
            let updated_file = file_service
                .update_file_by_id(&param_file_id, file_model, vec![])
                .await?;
            Ok(Web::ok("Update file successfully", updated_file))
        }
    }
}
