        cookie::get_cookie_user_id,
        depot::{get_file_service, get_user_service},
        file::get_file_from_req,
    response::FinalFileResponse,
    // Find the user
    let cookie_user = get_user_service(depot)?
        .get_user_by_id(cookie_user_id)
        .await?;

    let file_model = file_req.into_file(&cookie_user, full_filename)?;
    let created_file = file_service.create_file(file_model, file_stream).await?;
    Ok(Web::ok(
        "Create file successfully",
        FinalFileResponse::new(created_file, cookie_user, vec![])?,
    ))