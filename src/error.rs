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
        match self {
            Error::Env(e) => {
                res.render(Web::internal_error("Env file error".to_string(), e));
                res.set_status_error(StatusError::internal_server_error())
            }
            Error::Var(e) => {
                res.render(Web::internal_error(
                    "Cannot load one of the environment variables".to_string(),
                    e,
                ));
                res.set_status_error(StatusError::internal_server_error())
            }
            Error::MongoDB(e) => {
                res.render(Web::internal_error(
                    "MongoDB has encountered an error".to_string(),
                    e,
                ));
                res.set_status_error(StatusError::internal_server_error())
            }
            Error::Generic(e) => {
                res.render(Web::internal_error("Generic error thrown".to_string(), e));
                res.set_status_error(StatusError::internal_server_error())
            }
            Error::Validation(e) => {
                res.render(Web::bad_request(
                    "Validation error thrown".to_string(),
                    extract_validation_error(&e),
                ));
                res.set_status_error(StatusError::bad_request())
            }
            Error::HttpParse(e) => {
                res.render(Web::bad_request("HttpParse error thrown".to_string(), e));
                res.set_status_error(StatusError::bad_request())
            }
            Error::Jwt(e) => {
                res.render(Web::internal_error("JWT error thrown".to_string(), e));
                res.set_status_error(StatusError::internal_server_error())
            }
            Error::ObjectId(e) => {
                res.render(Web::internal_error("JWT error thrown".to_string(), e));
                res.set_status_error(StatusError::internal_server_error())
            }
            Error::Permissions(e) => {
                res.render(Web::forbidden("Permissions error thrown".to_string(), e));
                res.set_status_error(StatusError::forbidden())
            }
            Error::Presign(e) => {
                res.render(Web::internal_error("Presign URL error".to_string(), e));
                res.set_status_error(StatusError::internal_server_error())
            }
            Error::PutObject(e) => {
                res.render(Web::internal_error("PutObject error".to_string(), e));
                res.set_status_error(StatusError::internal_server_error())
            }
            Error::GetObject(e) => {
                res.render(Web::internal_error("GetObject error".to_string(), e));
                res.set_status_error(StatusError::internal_server_error())
            }
            Error::IO(e) => {
                res.render(Web::internal_error("IO error".to_string(), e));
                res.set_status_error(StatusError::internal_server_error())
            }
            Error::Aws(e) => {
                res.render(Web::internal_error("AWS error".to_string(), e));
                res.set_status_error(StatusError::internal_server_error())
            }
            Error::ListObject(e) => {
                res.render(Web::internal_error("ListObject error".to_string(), e));
                res.set_status_error(StatusError::internal_server_error())
            }
            Error::CopyObject(e) => {
                res.render(Web::internal_error("CopyObject error".to_string(), e));
                res.set_status_error(StatusError::internal_server_error())
            }
            Error::DeleteObject(e) => {
                res.render(Web::internal_error("DeleteObject error".to_string(), e));
                res.set_status_error(StatusError::internal_server_error())
            }
            Error::DeleteObjects(e) => {
                res.render(Web::internal_error("DeleteObjects error".to_string(), e));
                res.set_status_error(StatusError::internal_server_error())
            }
        }
    }
}
