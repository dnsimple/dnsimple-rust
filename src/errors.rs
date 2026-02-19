use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

/// Represents the possible errors thrown while interacting with the DNSimple API
#[derive(Error, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum DNSimpleError {
    #[error("Authentication failed")]
    Unauthorized,
    #[error("Bad Gateway")]
    BadGateway,
    #[error("{message}")]
    BadRequest {
        message: String,
        attribute_errors: Option<Value>,
    },
    #[error("{0}")]
    GatewayTimeout(String),
    #[error("Method not Allowed")]
    MethodNotAllowed,
    #[error("{0}")]
    NotFound(String),
    #[error("Your account is not subscribed or not in good standing")]
    PaymentRequired,
    #[error("{0}")]
    PreconditionRequired(String),
    #[error("Service Unavailable")]
    ServiceUnavailable,
    #[error("You exceeded the allowed number of requests per hour and your request has temporarily been throttled.")]
    TooManyRequests,
    #[error("Transport Error - {0}({1})")]
    Transport(String, String),
    #[error("Deserialization Error {0}")]
    Deserialization(String),
}

impl DNSimpleError {
    pub fn parse_response(code: u16, body: Option<Value>) -> DNSimpleError {
        match code {
            400 => Self::bad_request(body),
            401 => Self::Unauthorized,
            402 => Self::PaymentRequired,
            404 => Self::not_found(body),
            405 => Self::MethodNotAllowed,
            428 => Self::precondition_required(body),
            429 => Self::TooManyRequests,
            502 => Self::BadGateway,
            503 => Self::ServiceUnavailable,
            504 => Self::gateway_timeout(body),
            _ => Self::Transport(code.to_string(), "Unknown error".to_string()),
        }
    }

    pub fn parse_reqwest_error(error: reqwest::Error) -> DNSimpleError {
        Self::Transport(error.to_string(), "Request error".to_string())
    }

    fn bad_request(body: Option<Value>) -> DNSimpleError {
        match body {
            Some(json) => Self::BadRequest {
                message: Self::message_in(&json),
                attribute_errors: Some(json["errors"].clone()),
            },
            None => Self::BadRequest {
                message: String::from("Bad Request"),
                attribute_errors: None,
            },
        }
    }

    fn gateway_timeout(body: Option<Value>) -> DNSimpleError {
        match body {
            Some(json) => Self::GatewayTimeout(Self::message_in(&json)),
            None => Self::GatewayTimeout(String::from("Gateway Timeout")),
        }
    }

    fn not_found(body: Option<Value>) -> DNSimpleError {
        match body {
            Some(json) => Self::NotFound(Self::message_in(&json)),
            None => Self::NotFound(String::from("Not Found")),
        }
    }

    fn precondition_required(body: Option<Value>) -> DNSimpleError {
        match body {
            Some(json) => Self::PreconditionRequired(Self::message_in(&json)),
            None => Self::PreconditionRequired(String::from("Precondition Required")),
        }
    }

    fn message_in(json: &Value) -> String {
        match json["message"].as_str() {
            None => String::from("Unable to parse error message"),
            Some(json_string) => json_string.to_string(),
        }
    }
}
