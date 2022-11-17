use salvo::Router;

use self::{file::file_routes, folder::folder_routes, user::user_routes};

pub mod file;
pub mod folder;
pub mod user;

pub fn routes() -> Router {
    Router::new()
        .push(user_routes())
        .push(file_routes())
        .push(folder_routes())
}
