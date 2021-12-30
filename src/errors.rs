use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use ureq::{Response, Transport};

/// Represents the Error message payload returned by the DNSimple API
#[derive(Debug, Deserialize, Serialize)]
pub struct APIErrorMessage {
    pub message: Option<String>,
    pub errors: Option<Value>,
}

#[derive(Error, Debug)]
pub enum DNSimpleError {
    #[error("Authentication error")]
    Authentication,
    #[error("Message: {0} errors: {:?}")]
    BadRequest(String, Option<Value>),
    #[error("Transport Error – {0}({0})")]
    Transport(String, String),
    #[error("Deserialization Error {0}")]
    Deserialization(String),
}

impl DNSimpleError {
    pub fn parse_response(code: u16, _response: Response) -> DNSimpleError {
        return Self::BadRequest(code.to_string(), None);
    }

    pub fn parse_transport(_transport: Transport) -> DNSimpleError {
        return Self::Transport("VROOM".into(), "VROOM".into());
    }
}
