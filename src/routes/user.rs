use salvo::Router;

use crate::{
    handler::{
        auth::{login::login_handler, logout::logout_handler, refresh::refresh_handler},
        user::{
            create::create_user_handler,
            delete::delete_user_handler,
            get::{get_user_handler, get_users_handler},
            profile::profile_handler,
            update::update_user_handler,
        },
    },
    middleware::auth::check_login,
};

pub fn user_routes() -> Router {
    Router::with_path("user")
        // /user/
        .push(get_users_route())
        // /user/create
        .push(create_user_route())
        // /user/login
        .push(login_route())
        // /user/logout
        .push(logout_route())
        // /user/refresh
        .push(refresh_route())
        // /user/profile
        .push(profile_route())
        // /user/update/<param_user_id>
        .push(update_user_route())
        // /user/delete/<param_user_id>
        .push(delete_user_route())
        // /user/<param_user_id>
        .push(get_user_route())
}

pub fn get_users_route() -> Router {
    Router::with_path("").get(get_users_handler)
}

pub fn get_user_route() -> Router {
    Router::with_path("<param_user_id>").get(get_user_handler)
}

pub fn create_user_route() -> Router {
    Router::with_path("register").post(create_user_handler)
}

pub fn update_user_route() -> Router {
    Router::with_path("update")
        .path("<param_user_id>")
        .hoop(check_login)
        .put(update_user_handler)
}

pub fn delete_user_route() -> Router {
    Router::with_path("delete")
        .path("<param_user_id>")
        .hoop(check_login)
        .delete(delete_user_handler)
}

pub fn profile_route() -> Router {
    Router::with_path("profile")
        .hoop(check_login)
        .get(profile_handler)
}

pub fn login_route() -> Router {
    Router::with_path("login").post(login_handler)
}

pub fn refresh_route() -> Router {
    Router::with_path("refresh")
        .hoop(check_login)
        .post(refresh_handler)
}

pub fn logout_route() -> Router {
    Router::with_path("logout")
        .hoop(check_login)
        .delete(logout_handler)
}
