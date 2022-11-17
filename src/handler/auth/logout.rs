use salvo::{
    handler,
    http::cookie::{time::Duration, Cookie},
    Depot, Request, Response,
};

use crate::{
    error::Error,
    helper::{cookie::get_cookie_user_id, depot::get_user_service},
    web::Web,
    WebResult,
};

#[handler]
pub async fn logout_handler(req: &Request, depot: &Depot, res: &mut Response) -> WebResult {
    // Get the cookie user id send from the check login middleware
    let cookie_user_id = get_cookie_user_id(depot)?;

    let access_jwt = req
        .cookie("accessToken")
        .ok_or_else(|| Error::Permissions("You are not logged in to logout".to_string()))?
        .value()
        .to_string();

    let refresh_jwt = req
        .cookie("refreshToken")
        .ok_or_else(|| Error::Permissions("You are not logged in to logout".to_string()))?
        .value()
        .to_string();

    // Get the user_db
    let user_service = get_user_service(depot)?;

    // Update the refresh token to none
    user_service
        .update_refresh_token(cookie_user_id, "")
        .await?;

    // Delete the cookies, by overriding them with cookies that are 1 nanoseconds of age
    res.cookies_mut().remove(
        Cookie::build("accessToken", access_jwt)
            .path("/")
            .max_age(Duration::minutes(30))
            .http_only(true)
            .finish(),
    );
    res.cookies_mut().remove(
        Cookie::build("refreshToken", refresh_jwt)
            .path("/")
            .max_age(Duration::hours(2))
            .http_only(true)
            .finish(),
    );

    Ok(Web::ok("Logout successfully", ()))
}
