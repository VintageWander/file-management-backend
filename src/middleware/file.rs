use crate::{
    helper::{
        cookie::get_cookie_user_id_option, depot::get_file_service, param::get_param_file_id,
    },
    Result,
};
use salvo::{handler, Depot, FlowCtrl, Request, Response};

#[handler]
pub async fn get_file_by_id_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) -> Result<()> {
    let file_service = get_file_service(depot)?;

    let param_file_id = get_param_file_id(req)?;

    // Get the public file first
    // If the owner of that public file matches with the login user
    // Return the actual file without limits
    let file = file_service.get_public_file_by_id(&param_file_id).await?;

    // Get the user (optional, we have to handle two user cases, logged in, and not logged in)
    let param_file = match get_cookie_user_id_option(depot) {
        Some(cookie_user_id) => match *cookie_user_id == file.owner {
            true => {
                file_service
                    .get_file_by_id_owner(&param_file_id, cookie_user_id)
                    .await?
            }
            false => file,
        },
        None => file,
    };

    depot.insert("param_file", param_file);
    ctrl.call_next(req, depot, res).await;

    Ok(())
}
