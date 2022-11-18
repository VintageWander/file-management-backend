use aws_sdk_s3::{
    error::{
        CopyObjectError, DeleteObjectError, DeleteObjectsError, GetObjectError, ListObjectsV2Error,
        PutObjectError,
    },
    types::SdkError,
};
use salvo::{prelude::StatusError, Piece};
use thiserror::Error;

use crate::{helper::print_validation::extract_validation_error, web::Web, WebResult};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Variable error: {0}")]
    Var(#[from] std::env::VarError),

    #[error("Env error: {0}")]
    Env(#[from] dotenv::Error),

    #[error("MongoDB error: {0}")]
    MongoDB(#[from] mongodb::error::Error),

    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Permissions error: {0}")]
    Permissions(String),

    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),

    #[error("HttpParse error: {0}")]
    HttpParse(#[from] salvo::http::ParseError),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("ObjectId parse error: {0}")]
    ObjectId(#[from] mongodb::bson::oid::Error),

    #[error("Presign config error: {0}")]
    Presign(#[from] aws_sdk_s3::presigning::config::Error),

    #[error("S3 PutObject error: {0}")]
    PutObject(#[from] Box<SdkError<PutObjectError>>),

    #[error("S3 GetObject error: {0}")]
    GetObject(#[from] Box<SdkError<GetObjectError>>),

    #[error("S3 ListObject error: {0}")]
    ListObject(#[from] Box<SdkError<ListObjectsV2Error>>),

    #[error("S3 CopyObject error: {0}")]
    CopyObject(#[from] Box<SdkError<CopyObjectError>>),

    #[error("S3 DeleteObject error: {0}")]
    DeleteObject(#[from] Box<SdkError<DeleteObjectError>>),

    #[error("S3 DeleteObjects error: {0}")]
    DeleteObjects(#[from] Box<SdkError<DeleteObjectsError>>),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("AWS HTTP error: {0}")]
    Aws(#[from] aws_smithy_http::byte_stream::Error),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Generic(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Generic(s.to_string())
    }
}

impl From<Error> for WebResult {
    fn from(e: Error) -> Self {
        Self::Err(e)
    }
}

impl From<SdkError<PutObjectError>> for Error {
    fn from(e: SdkError<PutObjectError>) -> Self {
        Error::PutObject(Box::new(e))
    }
}

impl From<SdkError<GetObjectError>> for Error {
    fn from(e: SdkError<GetObjectError>) -> Self {
        Error::GetObject(Box::new(e))
    }
}

impl From<SdkError<ListObjectsV2Error>> for Error {
    fn from(e: SdkError<ListObjectsV2Error>) -> Self {
        Error::ListObject(Box::new(e))
    }
}

impl From<SdkError<CopyObjectError>> for Error {
    fn from(e: SdkError<CopyObjectError>) -> Self {
        Error::CopyObject(Box::new(e))
    }
}

impl From<SdkError<DeleteObjectError>> for Error {
    fn from(e: SdkError<DeleteObjectError>) -> Self {
        Error::DeleteObject(Box::new(e))
    }
}

impl From<SdkError<DeleteObjectsError>> for Error {
    fn from(e: SdkError<DeleteObjectsError>) -> Self {
        Error::DeleteObjects(Box::new(e))
    }
}

impl Piece for Error {
    fn render(self, res: &mut salvo::Response) {
        let error_message = match self {
            Error::Var(ref e) => format!("Cannot load one of the environment variables {e}"),
            Error::Env(ref e) => format!("Env file error: {e}"),
            Error::MongoDB(ref e) => format!("MongoDB has encountered an error {e}"),
            Error::Generic(ref e) => format!("Generic error: {e}"),
            Error::Permissions(ref e) => format!("Permission error: {e}"),
            Error::Validation(ref e) => {
                format!("Validation error {}", extract_validation_error(e))
            }
            Error::HttpParse(ref e) => format!("Http Parse error: {e}"),
            Error::Jwt(ref e) => format!("JWT error {e}"),
            Error::ObjectId(ref e) => format!("ObjectId parse error: {e}"),
            Error::Presign(ref e) => format!("Presign request error: {e}"),
            Error::PutObject(ref e) => format!("PutObject error {e}"),
            Error::GetObject(ref e) => format!("GetObject error {e}"),
            Error::ListObject(ref e) => format!("ListObject error {e}"),
            Error::CopyObject(ref e) => format!("Copy Object error {e}"),
            Error::DeleteObject(ref e) => format!("DeleteObject error {e}"),
            Error::DeleteObjects(ref e) => format!("DeleteObjects error {e}"),
            Error::IO(ref e) => format!("IO error {e}"),
            Error::Aws(ref e) => format!("Aws error {e}"),
        };

        let error_status = match self {
            Error::Permissions(_) => StatusError::forbidden(),
            Error::Generic(_) | Error::Validation(_) | Error::HttpParse(_) | Error::Jwt(_) => {
                StatusError::bad_request()
            }
            _ => StatusError::internal_server_error(),
        };

        let error = match self {
            Error::Permissions(_) => Web::forbidden(error_message),
            Error::Generic(_) | Error::Validation(_) | Error::HttpParse(_) | Error::Jwt(_) => {
                Web::bad_request(error_message)
            }
            _ => Web::internal_error(error_message),
        };

        res.render(error);
        res.set_status_error(error_status);
    }
}
