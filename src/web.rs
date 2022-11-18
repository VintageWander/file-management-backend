use salvo::{prelude::StatusCode, writer::Json, Piece, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::WebResult;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Web {
    code: String,
    message: String,
    data: Value,
    error: String,
}

impl Piece for Web {
    fn render(self, res: &mut Response) {
        res.render(Json(self));
        res.set_status_code(StatusCode::OK)
    }
}

impl From<Web> for WebResult {
    fn from(w: Web) -> Self {
        Self::Ok(w)
    }
}

impl Web {
    pub fn new<T: for<'a> Deserialize<'a> + Serialize>(
        code: StatusCode,
        message: &str,
        data: T,
        error: impl ToString,
    ) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            data: json!(&data),
            error: error.to_string(),
        }
    }

    pub fn ok<T: for<'a> Deserialize<'a> + Serialize>(message: impl ToString, data: T) -> Web {
        Self {
            code: StatusCode::OK.to_string(),
            message: message.to_string(),
            data: json!(&data),
            error: "".to_string(),
        }
    }

    pub fn bad_request(error: impl ToString) -> Web {
        Self {
            code: StatusCode::BAD_REQUEST.to_string(),
            message: String::default(),
            data: json!(&()),
            error: error.to_string(),
        }
    }

    pub fn internal_error(error: impl ToString) -> Web {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
            message: String::default(),
            data: json!(&()),
            error: error.to_string(),
        }
    }
    pub fn forbidden(error: impl ToString) -> Web {
        Self {
            code: StatusCode::FORBIDDEN.to_string(),
            message: String::default(),
            data: json!(&()),
            error: error.to_string(),
        }
    }
}
