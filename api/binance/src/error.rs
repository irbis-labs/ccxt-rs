use std::{io, time};

//use failure::Fail;
use reqwest;
use serde_json;
use url;
use websocket::result::WebSocketError;

#[derive(Clone, Debug)]
pub enum ServiceError {
    ServerError,
    ServiceUnavailable,
}

#[derive(Clone, Debug)]
pub enum ClientError {
    Unauthorized,
}

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        ClientError(err: ClientError)
        ServiceError(err: ServiceError)
        UnknownStatus(status: reqwest::StatusCode)
     }

    foreign_links {
        ReqError(reqwest::Error);
        InvalidHeaderError(reqwest::header::InvalidHeaderValue);
        IoError(io::Error);
        UrlParserError(url::ParseError);
        Json(serde_json::Error);
        TimestampError(time::SystemTimeError);
        WebSocketError(WebSocketError);
    }
}
