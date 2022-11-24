use salvo::{
    handler,
    http::cookie::{time::Duration, Cookie},
    Depot, Request, Response,
};

use crate::{
    error::Error,
    helper::{
        depot::get_user_service,
        jwt::{decode_jwt, encode_jwt, JwtType},
    },
    web::Web,
    WebResult,
};

#[handler]
pub async fn refresh_handler(req: &Request, depot: &Depot, res: &mut Response) -> WebResult {
    // Get the refresh token from the cookie, there are two cases might happen
    let refresh_token = req
        .cookie("refreshToken")
        .ok_or_else(|| Error::Permissions("You haven't logged in to perform this action".into()))?;

    // If there IS a cookie named refreshToken

    // Get the token, decode it, and then get the id back
    let cookie_user_id = decode_jwt(refresh_token.value().to_string(), JwtType::Refresh)?;

    // Get the user db
    let user_service = get_user_service(depot)?;

    // Get the user that is associated with the id in the token
    let cookie_user = user_service.get_user_by_id(&cookie_user_id).await?;

    // Generate a new access token
    let access_jwt = encode_jwt(&cookie_user, JwtType::Access)?;

    // Delete the old cookie if there is one
    res.cookies_mut().remove(
        Cookie::build("accessToken", access_jwt.clone())
            .path("/")
            .max_age(Duration::minutes(30))
            .http_only(true)
            .finish(),
    );

    // Append the new access token into the cookies
    res.add_cookie(
        Cookie::build("accessToken", access_jwt)
            .path("/")
            .max_age(Duration::minutes(30))
            .http_only(true)
            .finish(),
    );

    Ok(Web::ok("Successfully refresh the access token", ()))
}
