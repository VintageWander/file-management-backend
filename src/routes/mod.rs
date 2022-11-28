use salvo::Router;

use crate::{
    handler::content::{get_content_handler, get_content_with_version_handler},
    middleware::{auth::check_login_middleware, file::get_file_by_id_middleware},
};

use self::{file::file_routes, folder::folder_routes, user::user_routes};

pub mod file;
pub mod folder;
pub mod user;

pub fn routes() -> Router {
    Router::new()
        .push(user_routes())
        .push(file_routes())
        .push(folder_routes())
        .push(
            Router::with_path("content/<param_file_id>")
                .hoop(check_login_middleware)
                .hoop(get_file_by_id_middleware)
                .get(get_content_handler)
                .push(
                    Router::with_path("versions/<version_number>")
                        .get(get_content_with_version_handler),
                ),
        )
}
