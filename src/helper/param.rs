use mongodb::bson::oid::ObjectId;
use salvo::Request;
use serde::Deserialize;

use crate::Result;

pub fn extract_from_param<'a, T: Deserialize<'a>>(req: &'a mut Request, key: &str) -> Result<T> {
    req.param::<T>(key)
        .ok_or_else(|| format!("Cannot get {} from param", key).into())
}

pub fn get_param_user_id(req: &mut Request) -> Result<ObjectId> {
    let param_user_id = extract_from_param(req, "param_user_id")?;
    Ok(param_user_id)
}

pub fn get_param_file_id(req: &mut Request) -> Result<ObjectId> {
    let param_file_id = extract_from_param(req, "param_file_id")?;
    Ok(param_file_id)
}

pub fn get_param_folder_id(req: &mut Request) -> Result<ObjectId> {
    let param_folder_id = extract_from_param(req, "param_folder_id")?;
    Ok(param_folder_id)
}

pub fn get_param_version_number(req: &mut Request) -> Result<i64> {
    let version_number = extract_from_param(req, "version_number")?;
    Ok(version_number)
}
