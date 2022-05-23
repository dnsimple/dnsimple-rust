use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Borrow;
use thiserror::Error;
use ureq::{Response, Transport};

#[derive(Error, Deserialize, Serialize, Debug)]
pub enum DNSimpleError {
    #[error("Authentication error")]
    Authentication,
    #[error("Message: {0}")]
    BadRequest(String, Option<Value>),
    #[error("Transport Error – {0}({0})")]
    Transport(String, String),
    #[error("Deserialization Error {0}")]
    Deserialization(String),
}

impl DNSimpleError {
    pub fn parse_response(_code: u16, response: Response) -> DNSimpleError {
        let json = response
            .into_json::<Value>()
            .map_err(|e| DNSimpleError::Deserialization(e.to_string()))
            .unwrap();
        let message = &json["message"];
        let errors = serde::__private::Option::Some(json["errors"].borrow().clone());

        return Self::BadRequest(message.to_string(), errors);
    }

    pub fn parse_transport(_transport: Transport) -> DNSimpleError {
        return Self::Transport("VROOM".into(), "VROOM".into());
    }
}
