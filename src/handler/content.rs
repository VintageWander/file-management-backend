use std::{
    fs::{remove_file, File},
    io::{BufWriter, Write},
};

use futures::TryStreamExt;
use salvo::{fs::NamedFile, handler, Depot, Request, Response};

use crate::{
    helper::{
        depot::{get_param_file, get_storage},
        param::get_param_version_number,
    },
    Result,
};

#[handler]
pub async fn get_content_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<()> {
    let param_file = get_param_file(depot)?;

    let storage = get_storage(depot)?;

    let mut data = storage
        .get_data_by_key(&format!(
            "{}.{}",
            param_file.id,
            param_file.extension_to_str()
        ))
        .await?;

    let local_file_path = &format!("downloads/{}", param_file.full_filename);

    let file = File::create(local_file_path)?;
    let mut buf_writer = BufWriter::new(file);
    while let Some(bytes) = data.try_next().await? {
        buf_writer.write_all(&bytes)?;
    }
    buf_writer.flush()?;

    NamedFile::builder(local_file_path)
        .attached_name(&param_file.full_filename)
        .send(req.headers(), res)
        .await;

    remove_file(local_file_path)?;

    Ok(())
}

#[handler]
pub async fn get_content_with_version_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<()> {
    let param_file = get_param_file(depot)?;

    let storage = get_storage(depot)?;

    let version_number = get_param_version_number(req)?;

    let mut data = storage
        .get_data_by_key(&format!(
            "{}/{}.{}",
            param_file.id,
            version_number,
            param_file.extension_to_str()
        ))
        .await?;

    let local_file_path = &format!("downloads/{}", param_file.full_filename);

    let file = File::create(local_file_path)?;
    let mut buf_writer = BufWriter::new(file);
    while let Some(bytes) = data.try_next().await? {
        buf_writer.write_all(&bytes)?;
    }
    buf_writer.flush()?;

    NamedFile::builder(local_file_path)
        .attached_name(&param_file.full_filename)
        .send(req.headers(), res)
        .await;

    remove_file(local_file_path)?;

    Ok(())
}
