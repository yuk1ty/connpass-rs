use once_cell::sync::Lazy;
use reqwest::{header::USER_AGENT, Client, Response, StatusCode};

use crate::{
    errors::{ConnpassCliError, ConnpassResult, HttpResponseError},
    query::Query,
    response::ConnpassResponse,
};

const BASE_URL: &str = "https://connpass.com/api/v1/event/";
static CRATE_USER_AGENT: Lazy<String> = Lazy::new(|| {
    format!(
        "connpass-rs/{} (+https://github.com/yuk1ty/connpass-rs)",
        env!("CARGO_PKG_VERSION")
    )
});

/// Async API client for accessing and fetching data from connpass API.
#[derive(Clone)]
pub struct ConnpassClient {
    client: Client,
}

impl Default for ConnpassClient {
    fn default() -> Self {
        ConnpassClient {
            client: Client::new(),
        }
    }
}

impl ConnpassClient {
    pub fn new() -> Self {
        ConnpassClient::default()
    }

    /// Initializes client with your own client.
    pub fn with_client(client: Client) -> Self {
        ConnpassClient { client }
    }

    /// Sends requests and gets response from API.
    /// The response is internally converted to `response::ConnpassResponse` with handling errors.
    pub async fn send_request(self, query: Query) -> ConnpassResult<ConnpassResponse> {
        let response = self
            .client
            .get(BASE_URL)
            .header(USER_AGENT, CRATE_USER_AGENT.as_str())
            .query(&query.make_reqwest_query())
            .send()
            .await
            .map_err(|err| ConnpassCliError::HttpResponse(HttpResponseError::ReqwestError(err)))?;
        self.handler(response).await
    }

    async fn handler(&self, res: Response) -> ConnpassResult<ConnpassResponse> {
        dbg!("response = {}", &res);
        match res.status() {
            StatusCode::OK => res.json::<ConnpassResponse>().await.map_err(|err| {
                ConnpassCliError::HttpResponse(HttpResponseError::JsonDecode(format!("{}", err)))
            }),
            StatusCode::FORBIDDEN => {
                Err(ConnpassCliError::HttpResponse(HttpResponseError::Forbidden))
            }
            StatusCode::INTERNAL_SERVER_ERROR => Err(ConnpassCliError::HttpResponse(
                HttpResponseError::InternalServerError,
            )),
            StatusCode::SERVICE_UNAVAILABLE => Err(ConnpassCliError::HttpResponse(
                HttpResponseError::ServiceUnavailable,
            )),
            s => Err(ConnpassCliError::HttpResponse(HttpResponseError::Various(
                format!("Unexpected response received: {:?} (status code)", s),
            ))),
        }
    }
}

#[cfg(feature = "blocking")]
pub mod blocking {
    use reqwest::{
        blocking::{Client, Response},
        header::USER_AGENT,
        StatusCode,
    };

    use crate::{
        errors::{ConnpassCliError, ConnpassResult, HttpResponseError},
        query::Query,
        response::ConnpassResponse,
    };

    use super::{BASE_URL, CRATE_USER_AGENT};

    /// Blocking API client for accessing and fetching data from connpass.com
    pub struct ConnpassClient {
        client: Client,
    }

    impl Default for ConnpassClient {
        fn default() -> Self {
            ConnpassClient {
                client: Client::new(),
            }
        }
    }

    impl ConnpassClient {
        pub fn new() -> Self {
            ConnpassClient::default()
        }

        /// Initializes client with your own client.
        pub fn with_client(client: Client) -> Self {
            ConnpassClient { client }
        }

        pub fn send_request(self, query: Query) -> ConnpassResult<ConnpassResponse> {
            let response = self
                .client
                .get(BASE_URL)
                .header(USER_AGENT, CRATE_USER_AGENT.as_str())
                .query(&query.make_reqwest_query())
                .send()
                .map_err(|err| {
                    ConnpassCliError::HttpResponse(HttpResponseError::ReqwestError(err))
                })?;

            self.handler(response)
        }

        fn handler(&self, res: Response) -> ConnpassResult<ConnpassResponse> {
            dbg!("response = {}", &res);
            match res.status() {
                StatusCode::OK => res.json::<ConnpassResponse>().map_err(|err| {
                    ConnpassCliError::HttpResponse(HttpResponseError::JsonDecode(format!(
                        "{}",
                        err
                    )))
                }),
                StatusCode::FORBIDDEN => {
                    Err(ConnpassCliError::HttpResponse(HttpResponseError::Forbidden))
                }
                StatusCode::INTERNAL_SERVER_ERROR => Err(ConnpassCliError::HttpResponse(
                    HttpResponseError::InternalServerError,
                )),
                StatusCode::SERVICE_UNAVAILABLE => Err(ConnpassCliError::HttpResponse(
                    HttpResponseError::ServiceUnavailable,
                )),
                s => Err(ConnpassCliError::HttpResponse(HttpResponseError::Various(
                    format!("Unexpected response received: {:?} (status code)", s),
                ))),
            }
        }
    }
}
