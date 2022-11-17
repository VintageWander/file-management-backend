use salvo::{handler, Depot, Request};

use crate::{
    helper::{body::extract_from_body, depot::get_user_service},
    request::user::create::CreateUserRequest,
    web::Web,
    WebResult,
};

#[handler]
pub async fn create_user_handler(req: &mut Request, depot: &mut Depot) -> WebResult {
    // Get the user_db
    let user_service = get_user_service(depot)?;

    // Parse the request
    let user = extract_from_body::<CreateUserRequest>(req)
        .await?
        .into_user()?;

    let new_user = user_service.create_user(user).await?.into_response()?;

    Ok(Web::ok("Create user successfully", new_user))
}
