use mongodb::bson::oid::ObjectId;
use salvo::Depot;

use crate::error::Error;

use crate::Result;

pub fn get_cookie_user_id(depot: &Depot) -> Result<&ObjectId> {
    let oid = get_cookie_user_id_option(depot)
        .ok_or_else(|| Error::Permissions("You have to be logged in".into()))?;
    Ok(oid)
}

pub fn get_cookie_user_id_option(depot: &Depot) -> Option<&ObjectId> {
    depot.get::<ObjectId>("cookie_user_id")
}
