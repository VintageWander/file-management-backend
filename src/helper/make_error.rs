use validator::ValidationError;

pub fn validation_message(msg: &'static str) -> ValidationError {
    let mut error = ValidationError::new("");
    error.message = Some(std::borrow::Cow::Borrowed(msg));
    error
}
