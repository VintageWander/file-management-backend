#![allow(dead_code, unused_variables)]

use aws::S3;
use db::{
    file_db::FileDB, file_version_db::FileVersionDB, folder_db::FolderDB, mongo::DB,
    user_db::UserDB,
};
use dotenv::dotenv;
use salvo::{affix, prelude::TcpListener, Router, Server};
use service::{
    file_service::FileService, folder_service::FolderService, user_service::UserService,
};
use web::Web;

mod aws;
mod base;
mod db;
mod error;
mod handler;
mod helper;
mod middleware;
mod request;
mod response;
mod routes;
mod service;
mod validation;
mod web;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult = Result<Web>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let db = DB::init().await?;
    let file_db = FileDB::init(&db);
    let folder_db = FolderDB::init(&db);
    let user_db = UserDB::init(&db);
    let s3 = S3::init()?;
    let file_version_db = FileVersionDB::init(&db);

    let user_service = UserService::init(&user_db, &file_db, &folder_db, &s3);
    let file_service = FileService::init(&file_db, &folder_db, &file_version_db, &s3);
    let folder_service = FolderService::init(&file_db, &folder_db, &s3);

    let router = Router::with_hoop(
        affix::insert("user_service", user_service)
            .insert("folder_service", folder_service)
            .insert("file_service", file_service),
    )
    .push(routes::routes());

    let listener = TcpListener::bind("127.0.0.1:7878");
    Server::new(listener).serve(router).await;
    Ok(())
}
