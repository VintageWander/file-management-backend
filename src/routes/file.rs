use salvo::Router;

use crate::{
    handler::{
        file::{
            create::create_file_handler,
            delete::delete_file_handler,
            get::{get_file_by_id_handler, get_files_handler},
            restore::restore_file_handler,
            update::update_file_handler,
        },
        version::{
            delete::delete_file_version_handler,
            get::{get_version_handler, get_versions_handler},
        },
    },
    middleware::{auth::check_login_middleware, file::get_file_by_id_middleware},
};

pub fn file_routes() -> Router {
    Router::with_path("file")
        .push(get_files_route()) // file/
        .push(create_file_route()) // file/create/
        .push(update_file_route()) // file/update/<param_file_id>
        .push(delete_file_route()) // file/delete/<param_file_id>
        .push(restore_file_route()) // file/<param_file_id>/versions/restore/<version_number>
        .push(delete_version_route()) // file/<param_file_id>/versions/delete/<version_number>
        .push(get_file_version_route()) // file/<param_file_id>/versions/<version_number>
        .push(get_file_versions_route()) // file/<param_file_id>/versions/
        .push(get_file_route()) // file/<param_file_id>
}

pub fn get_files_route() -> Router {
    Router::new()
        .hoop(check_login_middleware)
        .get(get_files_handler)
}

pub fn get_file_route() -> Router {
    Router::with_path("<param_file_id>")
        .hoop(check_login_middleware)
        .hoop(get_file_by_id_middleware)
        .get(get_file_by_id_handler)
}

pub fn create_file_route() -> Router {
    Router::with_path("create")
        .hoop(check_login_middleware)
        .post(create_file_handler)
}

pub fn update_file_route() -> Router {
    Router::with_path("update/<param_file_id>")
        .hoop(check_login_middleware)
        .hoop(get_file_by_id_middleware)
        .put(update_file_handler)
}

pub fn delete_file_route() -> Router {
    Router::with_path("delete/<param_file_id>")
        .hoop(check_login_middleware)
        .hoop(get_file_by_id_middleware)
        .delete(delete_file_handler)
}

pub fn get_file_versions_route() -> Router {
    Router::with_path("<param_file_id>/versions")
        .hoop(check_login_middleware)
        .hoop(get_file_by_id_middleware)
        .get(get_versions_handler)
}

pub fn get_file_version_route() -> Router {
    Router::with_path("<param_file_id>/versions/<version_number>")
        .hoop(check_login_middleware)
        .hoop(get_file_by_id_middleware)
        .get(get_version_handler)
}

pub fn restore_file_route() -> Router {
    Router::with_path("<param_file_id>/versions/restore/<version_number>")
        .hoop(check_login_middleware)
        .hoop(get_file_by_id_middleware)
        .put(restore_file_handler)
}

pub fn delete_version_route() -> Router {
    Router::with_path("<param_file_id>/versions/delete/<version_number>")
        .hoop(check_login_middleware)
        .hoop(get_file_by_id_middleware)
        .delete(delete_file_version_handler)
}
