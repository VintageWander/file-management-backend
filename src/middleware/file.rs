use crate::{
    helper::{cookie::get_cookie_user_id, depot::get_file_service, param::get_param_file_id},
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
    let cookie_user_id = get_cookie_user_id(depot)?;
    let param_file_id = get_param_file_id(req)?;

    let param_file = get_file_service(depot)?
        .get_file_by_id_owner(&param_file_id, cookie_user_id)
        .await?;

    depot.insert("param_file", param_file);
    ctrl.call_next(req, depot, res).await;

    Ok(())
}
