use salvo::{handler, Depot, FlowCtrl, Request, Response};

use crate::{
    error::Error,
    helper::{
        depot::get_user_service,
        jwt::{decode_jwt, JwtType},
    },
    Result,
};

#[handler]
pub async fn check_login_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) -> Result<()> {
    // Get the access jwt token from the cookies
    // If there isn't, move on to the handler
    // When the handler asks for the cookie_user_id
    // There isn't any, because the user isn't logged in
    let Some(access_jwt) = req.cookie("accessToken") else {
        ctrl.call_next(req, depot, res).await;
        return Ok(());
    };

    // After getting the accessToken from the cookie, we need to decode it
    // And get the user id from the cookie
    // We name it cookie user id
    let Ok(cookie_user_id) =
        decode_jwt(access_jwt.value().to_string(), JwtType::Access) else {
            // If the access token is expired, we move on to the next route
            ctrl.call_next(req, depot, res).await;
            return Ok(())
        };

    // We have to check if the id maps to a user in the db

    // Get the user_service
    let user_service = get_user_service(depot)?;

    // Search for the user, of course if this fails an error will be thrown
    // Because this means that the user id in the cookie is gibberish data
    let cookie_user = user_service.get_user_by_id(&cookie_user_id).await?;

    // Get the refresh token from the cookie
    let refresh_jwt = 
            req
                .cookie("refreshToken")
                .ok_or_else(|| {
                    Error::Permissions(
                        "There is no refresh token going along with the access token. Cannot be authenticated".into(),
                    )
                })?
                .value()
                .to_string();

    // If the refresh token is empty, reject the request
    if refresh_jwt.is_empty() {
        return Err(Error::Permissions(
            "The refresh token cannot be empty".into(),
        ));
    }

    // Compare against the refresh token in the found user
    // Since if the user has already logged out,
    // making the refresh token inside the user model null
    // In which it cannot match with the user's refresh token field
    // We also checked for null token input in case that is a workaround our logout mechanism

    if refresh_jwt != *cookie_user.refresh_token {
        return Err(Error::Permissions(
                    "The refresh token in the cookie does not match the user's refresh token. Please login and try again"
                        .to_string(),
                ));
    }

    // Insert the cookie user id and the cookie user in the depot
    depot.insert("cookie_user_id", cookie_user_id);
    depot.insert("cookie_user", cookie_user);

    ctrl.call_next(req, depot, res).await;

    // If there is no cookie, this means that the user might be logged out,
    // Or the cookie expires
    // The process still goes on, because if the handler asks for the cookie
    // It will get None, and it better than converting the "get cookie statement" into a result
    // That would mean the entire middleware chain will stop, just because the user isn't logged in
    // The handler will be the functions deciding on whether or not an error is thrown
    // just because there is no user logged in

    Ok(())
}
