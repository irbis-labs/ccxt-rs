use std::io;
use std::mem;
use std::rc::Rc;

use futures::{Future, TryFutureExt};
use futures::compat::{Future01CompatExt, Stream01CompatExt, };
use futures::future::ready;
use futures::stream::Stream;
use futures01::Stream as _;
use reqwest::r#async::{Client as AsyncClient, Response as AsyncResponse, RequestBuilder as AsyncRequestBuilder, Decoder};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use url::Url;
use websocket::{ClientBuilder, OwnedMessage, WebSocketError};

use crate::error::*;

/// The base enpoint.
pub const API_BASE: &str = "https://api.binance.com";
pub const STREAM_BASE: &str = "wss://stream.binance.com:9443/ws/";

/// API credentials.
#[derive(Clone, Default)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Keys {
    api: String,
    secret: String,
}

/// API config.
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub keys: Keys,
    #[serde(with = "url_serde")]
    pub api_base: Url,
    #[serde(with = "url_serde")]
    pub stream_base: Url,
}

/// API client.
#[derive(Clone, Default)]
pub struct Client {
    inner: Rc<ClientInner>,
}

#[derive(Default)]
struct ClientInner {
    config: Config,
}

impl Keys {
    pub fn new(api: Option<String>, secret: Option<String>) -> Self {
        Keys {
            api: api.unwrap_or_default(),
            secret: secret.unwrap_or_default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let keys = Keys::default();
        let api_base = Url::parse(API_BASE).unwrap();
        let stream_base = Url::parse(STREAM_BASE).unwrap();
        Config { keys, api_base, stream_base}
    }
}

pub struct RequestBuilder {
    client: Client,
    request: AsyncRequestBuilder,
}

pub struct WebsocketBuilder {
    client: Client,
    url: Url,
}

impl Client {
    pub fn new() -> Self {
        Client::default()
    }

    pub fn with_config(config: Config) -> Self {
        let inner = Rc::new(ClientInner { config });
        Client { inner }
    }

    pub fn get(&self, endpoint: &str) -> Result<RequestBuilder> {
        let url = self.inner.config.api_base.join(endpoint)?;
        let client = self.clone();
        Ok(RequestBuilder { client, request: AsyncClient::new().get(url) })
    }

    pub fn post(&self, endpoint: &str) -> Result<RequestBuilder> {
        let url = self.inner.config.api_base.join(endpoint)?;
        let client = self.clone();
        Ok(RequestBuilder { client, request: AsyncClient::new().post(url) })
    }

    pub fn put(&self, endpoint: &str) -> Result<RequestBuilder> {
        let url = self.inner.config.api_base.join(endpoint)?;
        let client = self.clone();
        Ok(RequestBuilder { client, request: AsyncClient::new().put(url) })
    }

    pub fn delete(&self, endpoint: &str) -> Result<RequestBuilder> {
        let url = self.inner.config.api_base.join(endpoint)?;
        let client = self.clone();
        Ok(RequestBuilder { client, request: AsyncClient::new().delete(url) })
    }

    pub fn web_socket(&self, name: &str) -> Result<WebsocketBuilder> {
        let url = self.inner.config.stream_base.join(name)?;
        let client = self.clone();
        Ok(WebsocketBuilder { client, url })
    }

    pub async fn web_socket2<'x>(&'x self, name: &'x str) -> Result<WebsocketBuilder> {
        let url = self.inner.config.stream_base.join(name)?;
        let client = self.clone();
        Ok(WebsocketBuilder { client, url })
    }
}

impl WebsocketBuilder {
    pub async fn connect<V>(self) -> Result<impl Stream<Item=Result<V>>>
    where
        V: serde::de::DeserializeOwned,
    {
        use futures01::{Future, Sink, Stream};
        ClientBuilder::new(self.url.as_str())
            .map_err(Error::from)?
            .async_connect(None)
            .map(move |(duplex, _)| {
                let (sink, stream) = duplex.split();
                let (tx, rx) = futures01::sync::mpsc::channel(0);
                let rx = sink
                    .sink_map_err(drop)
                    .send_all(rx.filter_map(|m| m).map_err(drop))
                    .map(drop);
                tokio::runtime::current_thread::spawn(rx);
                let stream = stream
                    .and_then(move |message| {
                        // dbg!("Received Message: {:?}", message);
                        let (up, dn) = match message {
                            OwnedMessage::Close(e) => (Some(OwnedMessage::Close(e)), None),
                            OwnedMessage::Ping(d) => (Some(OwnedMessage::Pong(d)), None),
                            OwnedMessage::Pong(_) => (None, None),
                            OwnedMessage::Text(msg) => (None, Some(msg)),
                            OwnedMessage::Binary(_d) => {
                                // warn!("Unexpected binary data {:?}", d);
                                (None, None)
                            }
                        };
                        tx.clone().send(up)
                            .map_err(|_| WebSocketError::IoError(io::ErrorKind::ConnectionAborted.into()))
                            .map(|_| dn)
                    })
                    .filter_map(|v: Option<String>| v)
                    .map_err(Error::from)
                    .and_then(|msg| serde_json::from_str(&msg).map_err(Error::from))
                    .compat();
                stream

            })
            .map_err(Error::from)
            .compat()
            .await
    }
}

impl RequestBuilder {
    pub fn query_args<T: Serialize + ?Sized>(mut self, query: &T) -> Self {
        self.request = self.request.query(query);
        self
    }

    pub fn query_arg<S: AsRef<str>, T: Serialize + ?Sized>(mut self, name: S, query: &T) -> Self {
        self.request = self.request.query(&[(name.as_ref(), query)]);
        self
    }

    pub fn try_query_arg<S: AsRef<str>, T: Serialize>(mut self, name: S, query: &Option<T>) -> Self {
        if let Some(val) = query {
            self.request = self.request.query(&[(name.as_ref(), val)]);
        }
        self
    }

    pub fn auth_header(mut self) -> Result<Self> {
        self.request = self.request.header(
            "X-MBX-APIKEY",
            reqwest::header::HeaderValue::from_str(self.client.inner.config.keys.api.as_str())?
        );
        Ok(self)
    }

    pub async fn send<V>(self) -> Result<V>
    where
        V: serde::de::DeserializeOwned,
    {
        let res = self.request.send().compat().await?;
        let mut res = check_response(res)?;
        let body = res.into_body().concat2().compat().await?;
        Ok(serde_json::from_slice(&body)?)
    }
}

fn check_response(res: AsyncResponse) -> Result<AsyncResponse> {
    match res.status() {
        StatusCode::OK => {
            Ok(res)
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            Err(ErrorKind::ServiceError(ServiceError::ServerError).into())
        }
        StatusCode::SERVICE_UNAVAILABLE => {
            Err(ErrorKind::ServiceError(ServiceError::ServiceUnavailable).into())
        }
        StatusCode::UNAUTHORIZED => {
            Err(ErrorKind::ClientError(ClientError::Unauthorized).into())
        }
//        StatusCode::BAD_REQUEST => {
//            let error_json: BinanceContentError = response.json()?;
//
//            Err(ErrorKind::BinanceError(error_json.code, error_json.msg, response).into())
//        }
        s => {
            Err(ErrorKind::UnknownStatus(s).into())
        }
    }
}

#[derive(Debug, Deserialize)]
struct BinanceContentError {
    pub code: i16,
    pub msg: String,
}

//enum WSClientState {
//    Connecting,
//    Disconnected,
//}
//
//pub struct WSClient {
//    state: WSClientState,
//    cb: ClientBuilder<'static>,
//}
//
//impl WSClient {
//    fn new(url: Url) -> Result<Self> {
//        let cb = ClientBuilder::new(url.as_str()).map_err(Error::from)?;
//        let state = WSClientState::Disconnected;
//        Ok(WSClient { cb, state })
//    }
//}
