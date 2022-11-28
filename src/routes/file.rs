use salvo::Router;

use crate::{
    handler::file::{
        create::create_file_handler,
        delete::delete_file_handler,
        get::{get_file_by_id_handler, get_public_files_handler},
        restore::restore_file_handler,
        update::update_file_handler,
    },
    middleware::auth::check_login,
};

pub fn file_routes() -> Router {
    Router::with_path("file")
        .push(get_public_files_route()) // file/
        .push(create_file_route()) // file/create/
        .push(update_file_route()) // file/update/<param_file_id>
        .push(delete_file_route()) // file/delete/<param_file_id>
        .push(restore_file_route()) // file/restore/<param_file_id>/<version_number>
        .push(get_file_route()) // file/<param_file_id>
}

pub fn get_public_files_route() -> Router {
    Router::new().get(get_public_files_handler)
}

pub fn get_file_route() -> Router {
    Router::with_path("<param_file_id>")
        .hoop(check_login)
        .get(get_file_by_id_handler)
}

pub fn create_file_route() -> Router {
    Router::with_path("create")
        .hoop(check_login)
        .post(create_file_handler)
}

pub fn update_file_route() -> Router {
    Router::with_path("update")
        .path("<param_file_id>")
        .hoop(check_login)
        .put(update_file_handler)
}

pub fn delete_file_route() -> Router {
    Router::with_path("delete")
        .path("<param_file_id>")
        .hoop(check_login)
        .delete(delete_file_handler)
}

pub fn restore_file_route() -> Router {
    Router::with_path("restore")
        // .path("<param_file_id>")
        // .path("<version_number>")
        .hoop(check_login)
        .put(restore_file_handler)
}
