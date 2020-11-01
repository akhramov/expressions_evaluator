use std::io::Cursor;

use rocket::{Request, Response, response::Result, http::Status, response::Responder};
use serde::Serialize;

/// Represents non-fatal server error.
/// We can't use anyhow, because it doesn't implement rocket's `Responder` trait.
#[derive(Debug, Serialize)]
pub struct Error {
    reason: String
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Self {
            reason: format!("{}", err),
        }
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> Result<'r> {
        Response::build()
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .status(Status::UnprocessableEntity)
            .ok()
    }
}
