use salvo::{
    handler,
    http::cookie::{time::Duration, Cookie},
    Depot, Request, Response,
};
use serde_json::json;

use crate::{
    helper::{
        body::extract_from_body,
        depot::get_user_service,
        jwt::{encode_jwt, JwtType},
    },
    request::user::login::LoginRequest,
    web::Web,
    WebResult,
};

#[handler]
pub async fn login_handler(req: &mut Request, depot: &mut Depot, res: &mut Response) -> WebResult {
    // Extract the login request and validate
    let user_req = extract_from_body::<LoginRequest>(req)
        .await?
        .validate_self()?;

    // Get the user_db
    let user_service = get_user_service(depot)?;

    // Find the user
    let cookie_user = user_service
        .get_user_by_login_info(&user_req.username, &user_req.password)
        .await?;

    // Get the id from the cookie user
    let cookie_user_id = cookie_user.id;

    // Create the access token and the refresh token
    let access_jwt = encode_jwt(&cookie_user, JwtType::Access)?;
    let refresh_jwt = encode_jwt(&cookie_user, JwtType::Refresh)?;

    // Get the cookie storage in response object, and put the tokens there
    res.cookies_mut().add(
        Cookie::build("accessToken", access_jwt.clone())
            .path("/")
            .max_age(Duration::minutes(30))
            .http_only(true)
            .finish(),
    );

    res.cookies_mut().add(
        Cookie::build("refreshToken", refresh_jwt.clone())
            .path("/")
            .max_age(Duration::hours(2))
            .http_only(true)
            .finish(),
    );

    // Update the refresh token field in the user
    user_service
        .update_refresh_token(&cookie_user_id, &refresh_jwt)
        .await?;

    Ok(Web::ok(
        "Login successfully",
        json!({
            "user": cookie_user.into_response()?,
            "accessToken": access_jwt,
            "refreshToken": refresh_jwt,
        }),
    ))
}
