use salvo::Depot;

use crate::{
    service::{
        file_service::FileService, folder_service::FolderService, user_service::UserService,
    },
    Result,
};

pub fn extract_from_depot<'a, T: Sync + Send + Clone + 'static>(
    depot: &'a Depot,
    key: &'a str,
) -> Result<&'a T> {
    depot
        .get::<T>(key)
        .ok_or_else(|| format!("Cannot get {} from depot", key).into())
}

pub fn extract_from_depot_option<'a, T: Sync + Send + Clone + 'static>(
    depot: &'a Depot,
    key: &'a str,
) -> Option<&'a T> {
    depot.get::<T>(key)
}

pub fn get_user_service(depot: &Depot) -> Result<&UserService> {
    extract_from_depot(depot, "user_service")
}

pub fn get_file_service(depot: &Depot) -> Result<&FileService> {
    extract_from_depot(depot, "file_service")
}

pub fn get_folder_service(depot: &Depot) -> Result<&FolderService> {
    extract_from_depot(depot, "folder_service")
}
