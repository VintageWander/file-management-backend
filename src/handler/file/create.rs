use salvo::{handler, Depot, Request};
        cookie::get_cookie_user, depot::get_file_service, file::get_file_from_req,
pub async fn create_file_handler(req: &mut Request, depot: &mut Depot) -> WebResult {
    let cookie_user = get_cookie_user(depot)?;
    let file_model = file_req.into_file(cookie_user, full_filename)?;
        FinalFileResponse::new(created_file, cookie_user.clone(), vec![])?,