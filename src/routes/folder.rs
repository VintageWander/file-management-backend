use salvo::Router;

use crate::{
    handler::folder::{
        create::create_folder_handler,
        delete::delete_folder_handler,
        get::{get_folder_by_id_handler, get_folders_handler},
        update::update_folder_handler,
    },
    middleware::{auth::check_login_middleware, folder::get_folder_by_id_middleware},
};

pub fn folder_routes() -> Router {
    Router::with_path("folder")
        .push(get_folders_route()) // folder/
        .push(create_folder_route()) // folder/create/
        .push(update_folder_route()) // folder/update/<param_folder_id>
        .push(delete_folder_route()) // folder/delete/<param_folder_id>
        .push(get_folder_route()) // folder/<param_folder_id>
}

pub fn get_folders_route() -> Router {
    Router::new()
        .hoop(check_login_middleware)
        .get(get_folders_handler)
}

pub fn get_folder_route() -> Router {
    Router::with_path("<param_folder_id>")
        .hoop(check_login_middleware)
        .hoop(get_folder_by_id_middleware)
        .get(get_folder_by_id_handler)
}

pub fn create_folder_route() -> Router {
    Router::with_path("create")
        .hoop(check_login_middleware)
        .post(create_folder_handler)
}

pub fn update_folder_route() -> Router {
    Router::with_path("update/<param_folder_id>")
        .hoop(check_login_middleware)
        .hoop(get_folder_by_id_middleware)
        .put(update_folder_handler)
}

pub fn delete_folder_route() -> Router {
    Router::with_path("delete/<param_folder_id>")
        .hoop(check_login_middleware)
        .hoop(get_folder_by_id_middleware)
        .delete(delete_folder_handler)
}
