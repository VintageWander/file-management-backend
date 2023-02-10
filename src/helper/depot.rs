use salvo::Depot;

use crate::{
    aws::S3,
    base::{file::File, folder::Folder},
    service::{
        file_service::FileService, file_version_service::FileVersionService,
        folder_service::FolderService, user_service::UserService,
    },
    Result,
};

pub fn extract_from_depot<'a, T: Sync + Send + Clone + 'static>(
    depot: &'a Depot,
    key: &'a str,
) -> Result<&'a T> {
    depot
        .get::<T>(key)
        .ok_or_else(|| format!("Cannot get {key} from depot").into())
}

// pub fn extract_from_depot_option<'a, T: Sync + Send + Clone + 'static>(
//     depot: &'a Depot,
//     key: &'a str,
// ) -> Option<&'a T> {
//     depot.get::<T>(key)
// }

// Getting from depot means 2 things
// Getting services injected into affix
// Getting values after the user has logged in
// Like get_param_file or get_param_folder guarantees that the user is logged in

// But get_param_file_id or get_param_folder_id takes the id out of the param
// That does not guarantees that the user is logged in
// I would suggest that if you want to get param id, use the functions here, and then .id

pub fn get_user_service(depot: &Depot) -> Result<&UserService> {
    extract_from_depot(depot, "user_service")
}

pub fn get_file_service(depot: &Depot) -> Result<&FileService> {
    extract_from_depot(depot, "file_service")
}

pub fn get_folder_service(depot: &Depot) -> Result<&FolderService> {
    extract_from_depot(depot, "folder_service")
}

pub fn get_storage(depot: &Depot) -> Result<&S3> {
    extract_from_depot(depot, "storage")
}

pub fn get_file_version_service(depot: &Depot) -> Result<&FileVersionService> {
    extract_from_depot(depot, "file_version_service")
}

pub fn get_param_file(depot: &Depot) -> Result<&File> {
    extract_from_depot(depot, "param_file")
}

pub fn get_param_folder(depot: &Depot) -> Result<&Folder> {
    extract_from_depot(depot, "param_folder")
}
