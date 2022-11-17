use salvo::{handler, Depot, Request};

use crate::{
    error::Error,
    helper::{
        body::extract_from_body, cookie::get_cookie_user_id, depot::get_user_service,
        param::get_param_user_id,
    },
    request::user::delete::DeleteUserRequest,
    web::Web,
    WebResult,
};

#[handler]
pub async fn delete_user_handler(req: &mut Request, depot: &mut Depot) -> WebResult {
    // Extract the delete user request
    let user_req: DeleteUserRequest = extract_from_body(req).await?;

    // Get the user db
    let user_service = get_user_service(depot)?;

    // Get the param user id
    let param_user_id = get_param_user_id(req)?;

    // Find the current user
    let param_user = user_service.get_user_by_id(&param_user_id).await?;

    let cookie_user_id = get_cookie_user_id(depot)?;

    if param_user_id != *cookie_user_id {
        return Err(Error::Permissions(
            "You cannot delete other user's profile".into(),
        ));
    }

    // Validate the request with current password
    user_req.validate_self(&param_user.password)?;

    // The above code checks if the password and confirmPassword are the same
    // and also check if the request password is the same as the user's current password

    // Finally delete the user
    user_service.delete_user_by_id(&param_user_id).await?;

    // We have successflly deleted a user
    Ok(Web::ok("Delete user successfully", ()))
}
