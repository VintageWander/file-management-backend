use salvo::{handler, Depot, Request, Response};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    helper::{
        cookie::get_cookie_user_id, depot::get_file_service, file::get_file_from_req,
        form::extract_from_form,
    },
    request::file::create::CreateFileRequest,
    web::Web,
    WebResult,
};

#[handler]
pub async fn create_file_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    // Checks if the user has logged in or not
    let cookie_user_id = get_cookie_user_id(depot)?;

    // Extract the data from request
    let file_req = extract_from_form::<CreateFileRequest>(req).await?;

    // Get the file service
    let file_service = get_file_service(depot)?;

    // Get the attachment file
    let file = get_file_from_req(req).await?;

    // Open the received file from temporary path
    let mut local_file = File::open(file.path()).await?;

    // Initialize the file stream
    let mut file_stream = vec![];

    // Read the file's byte stream and store it inside file_stream
    local_file.read_to_end(&mut file_stream).await?;

    let full_filename = file
        .name()
        .ok_or("The attached file does not have a name")?;

    // Construct the file model from request
    let file_model = file_req.into_file(cookie_user_id, full_filename)?;

    // Send the file_model and the file_stream to the database to create a new file model
    // with the file stream send straight to S3
    let created_file = file_service
        .create_file(file_model, file_stream)
        .await?
        .into_response()?;

    // Return back the created file
    Ok(Web::ok("Create file successfully", created_file))
}
