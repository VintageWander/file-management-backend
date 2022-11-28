use crate::{
    helper::{
        cookie::get_cookie_user_id_option, depot::get_folder_service, param::get_param_folder_id,
    },
    Result,
};
use salvo::{handler, Depot, FlowCtrl, Request, Response};

#[handler]
pub async fn get_folder_by_id_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) -> Result<()> {
    let folder_service = get_folder_service(depot)?;

    let param_folder_id = get_param_folder_id(req)?;

    let folder = folder_service
        .get_public_folder_by_id(&param_folder_id)
        .await?;

    // Get the user (optional, we have to handle two user cases, logged in, and not logged in)
    let param_folder = match get_cookie_user_id_option(depot) {
        Some(cookie_user_id) => match *cookie_user_id == folder.owner {
            true => {
                folder_service
                    .get_public_folder_by_id(&param_folder_id)
                    .await?
            }
            false => folder,
        },
        None => folder,
    };

    depot.insert("param_folder", param_folder);
    ctrl.call_next(req, depot, res).await;

    Ok(())
}
