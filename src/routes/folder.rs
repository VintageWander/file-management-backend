use salvo::Router;

use crate::{
    handler::folder::{
        create::create_folder_handler,
        delete::delete_folder_handler,
        get::{get_folder_by_id_handler, get_public_folders_handler},
        update::update_folder_handler,
    },
    middleware::auth::check_login,
};

pub fn folder_routes() -> Router {
    Router::with_path("folder")
        .push(get_public_folders_route()) // folder/
        .push(create_folder_route()) // folder/create/
        .push(update_folder_route()) // folder/update/<param_folder_id>
        .push(delete_folder_route()) // folder/delete/<param_folder_id>
        .push(get_folder_route()) // folder/<param_folder_id>
}

pub fn get_public_folders_route() -> Router {
    Router::new().get(get_public_folders_handler)
}

pub fn get_folder_route() -> Router {
    Router::with_path("<param_folder_id>")
        .hoop(check_login)
        .get(get_folder_by_id_handler)
}

pub fn create_folder_route() -> Router {
    Router::with_path("create")
        .hoop(check_login)
        .post(create_folder_handler)
}

pub fn update_folder_route() -> Router {
    Router::with_path("update")
        .path("<param_folder_id>")
        .hoop(check_login)
        .put(update_folder_handler)
}

pub fn delete_folder_route() -> Router {
    Router::with_path("delete")
        .path("<param_folder_id>")
        .hoop(check_login)
        .delete(delete_folder_handler)
}