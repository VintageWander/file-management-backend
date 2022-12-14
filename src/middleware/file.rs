use crate::{
    helper::{
        cookie::get_cookie_user_id_option, depot::get_file_service, param::get_param_file_id,
    },
    Result,
};
use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use salvo::{handler, Depot, FlowCtrl, Request, Response};

#[handler]
pub async fn get_file_by_id_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) -> Result<()> {
    // The only case where I have to clone the service
    let file_service = get_file_service(depot)?.clone();

    let param_file_id = get_param_file_id(req)?;

    let file = file_service.get_file_by_id(&param_file_id);
    let param_file = file_service.get_public_file_by_id(&param_file_id);


    // Note
    // In this code, file is the one that could be private
    // param_file, only is the public file, if the request file is private
    // it will contains nothing

    // Get the actual file first
    // If there is no user logged in, which means that the user is the guest
    // Then return to them a file with that id but only if it is public
    
    // Get the user (if the user is a guest, return the file only if it is public, else it will show not found)
    let Some(cookie_user_id) = get_cookie_user_id_option(depot) else {
        // If the user is a guest, this checks for 2 query strings
        // One is the owner id
        // Another is the expiry time (in miliseconds)
        
        let (Some(owner), Some(expiry_time)) = (req.query::<ObjectId>("owner"), req.query::<i64>("expiry_time")) else {
            // If any of the two fields are omitted, then return the public file

            let param_file = param_file.await?;
        
            depot.insert("param_file", param_file);
            ctrl.call_next(req, depot, res).await;
            return Ok(());
        };

        // After those checks
        // Now we check if the owner query string matches the actual owner of the file
        // And if the expiry time is still remaining

        let file = file.await?;

        if (owner != file.owner) || (Utc::now().timestamp_millis() > expiry_time) {
            let param_file = param_file.await?;

            depot.insert("param_file", param_file);
            ctrl.call_next(req, depot, res).await;
            return Ok(());
        }
        
        depot.insert("param_file", file);
        ctrl.call_next(req, depot, res).await;
        return Ok(());
    };

    let file = file.await?;

    let param_file = if *cookie_user_id == file.owner {
        file
    } else {
        param_file.await?
    };

    depot.insert("param_file", param_file);
    ctrl.call_next(req, depot, res).await;

    Ok(())
}
