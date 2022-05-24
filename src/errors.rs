use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Borrow;
use thiserror::Error;
use ureq::{Response, Transport};

#[derive(Error, Deserialize, Serialize, Debug)]
pub enum DNSimpleError {
    #[error("Authentication failed")]
    Unauthorized,
    #[error("Bad Gateway")]
    BadGateway,
    #[error("Message: {0}")]
    BadRequest(String, Option<Value>),
    #[error("Message: {0}")]
    GatewayTimeout(String),
    #[error("Method not Allowed")]
    MethodNotAllowed,
    #[error("Message: {0}")]
    NotFound(String),
    #[error("Your account is not subscribed or not in good standing")]
    PaymentRequired,
    #[error("Message: {0}")]
    PreconditionRequired(String),
    #[error("Service Unavailable")]
    ServiceUnavailable,
    #[error("You exceeded the allowed number of requests per hour and your request has temporarily been throttled.")]
    TooManyRequests,
    #[error("Transport Error – {0}({0})")]
    Transport(String, String),
    #[error("Deserialization Error {0}")]
    Deserialization(String),
}

impl DNSimpleError {
    pub fn parse_response(code: u16, response: Response) -> DNSimpleError {
        match code {
            400 => Self::bad_request(response),
            401 => Self::Unauthorized,
            402 => Self::PaymentRequired,
            404 => Self::not_found(response),
            405 => Self::MethodNotAllowed,
            428 => Self::precondition_required(response),
            429 => Self::TooManyRequests,
            502 => Self::BadGateway,
            503 => Self::ServiceUnavailable,
            504 => Self::gateway_timeout(response),
            _ => Self::Transport("OOPS".into(), "oops".into()),
        }
    }

    pub fn parse_transport(_transport: Transport) -> DNSimpleError {
        Self::Transport("VROOM".into(), "VROOM".into())
    }

    fn bad_request(response: Response) -> DNSimpleError {
        let json = Self::response_to_json(response);

        Self::BadRequest(
            json["message"].to_string(),
            Some(json["errors"].borrow().clone()),
        )
    }

    fn gateway_timeout(response: Response) -> DNSimpleError {
        let json = Self::response_to_json(response);

        Self::GatewayTimeout(json["message"].to_string())
    }

    fn not_found(response: Response) -> DNSimpleError {
        let json = Self::response_to_json(response);

        Self::NotFound(json["message"].to_string())
    }

    fn precondition_required(response: Response) -> DNSimpleError {
        let json = Self::response_to_json(response);

        Self::PreconditionRequired(json["message"].to_string())
    }

    fn response_to_json(response: Response) -> Value {
        response
            .into_json::<Value>()
            .map_err(|e| DNSimpleError::Deserialization(e.to_string()))
            .unwrap()
    }
}
