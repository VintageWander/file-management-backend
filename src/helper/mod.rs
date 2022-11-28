pub mod body;
pub mod comparison;
pub mod cookie;
pub mod depot;
pub mod file;
pub mod form;
pub mod jwt;
pub mod make_error;
pub mod param;
pub mod position;
pub mod print_validation;
// pub mod versioning;

pub fn into_string<T: ToString>(item: T) -> String {
    item.to_string()
}
