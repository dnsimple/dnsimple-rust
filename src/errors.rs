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
    #[error("Message: {0}")]
    GatewayTimeout(String),
    #[error("Transport Error – {0}({0})")]
    Transport(String, String),
    #[error("Deserialization Error {0}")]
    Deserialization(String),
}

impl DNSimpleError {
    pub fn parse_response(code: u16, response: Response) -> DNSimpleError {
        if code == 400 {
            Self::bad_request(response)
        } else if code == 504 {
            Self::gateway_timeout(response)
        } else {
            Self::Transport("OOPS".into(), "oops".into())
        }
    }

    pub fn parse_transport(_transport: Transport) -> DNSimpleError {
        Self::Transport("VROOM".into(), "VROOM".into())
    }

    fn bad_request(response: Response) -> DNSimpleError {
        let json = response
            .into_json::<Value>()
            .map_err(|e| DNSimpleError::Deserialization(e.to_string()))
            .unwrap();
        let message = &json["message"];
        let errors = Some(json["errors"].borrow().clone());

        Self::BadRequest(message.to_string(), errors)
    }

    fn gateway_timeout(response: Response) -> DNSimpleError {
        let json = response
            .into_json::<Value>()
            .map_err(|e| DNSimpleError::Deserialization(e.to_string()))
            .unwrap();
        let message = &json["message"];

        Self::GatewayTimeout(message.to_string())
    }
}
