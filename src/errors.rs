use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Borrow;
use thiserror::Error;
use ureq::{Response, Transport};

/// Represents the possible errors thrown while interacting with the DNSimple API
#[derive(Error, Deserialize, Serialize, Debug, PartialEq)]
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
    #[error("Transport Error – {0}({1})")]
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
            _ => Self::Transport(
                response.status().to_string(),
                response.status_text().to_string(),
            ),
        }
    }

    pub fn parse_transport(transport: Transport) -> DNSimpleError {
        Self::Transport(transport.to_string(), transport.kind().to_string())
    }

    fn bad_request(response: Response) -> DNSimpleError {
        let json = Self::response_to_json(response);

        Self::BadRequest(
            Self::message_in(&json),
            Some(json["errors"].borrow().clone()),
        )
    }

    fn gateway_timeout(response: Response) -> DNSimpleError {
        Self::GatewayTimeout(Self::message_in(&Self::response_to_json(response)))
    }

    fn not_found(response: Response) -> DNSimpleError {
        Self::NotFound(Self::message_in(&Self::response_to_json(response)))
    }

    fn precondition_required(response: Response) -> DNSimpleError {
        Self::PreconditionRequired(Self::message_in(&Self::response_to_json(response)))
    }

    fn message_in(json: &Value) -> String {
        json["message"].as_str().unwrap().to_string()
    }

    fn response_to_json(response: Response) -> Value {
        response
            .into_json::<Value>()
            .map_err(|e| DNSimpleError::Deserialization(e.to_string()))
            .unwrap()
    }
}
