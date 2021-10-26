//! Sends requets to connpass API server with queries.
//! This module provides non-blocking API (on tokio runtime) normally, but when `blocking` feature is enabled, additionally start to provide blocking API.
//! These clients are internally using `reqwest` crate.

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
    ///
    /// # Arguments
    /// If no condition is set to `query` and it's passed, the default options are applied.
    /// The defaults are described in the connpass API specification page.
    ///
    /// # Example:
    /// ```
    /// use connpass_rs::{client::ConnpassClient, query::builder::QueryBuilder};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     // fetch https://rust.connpass.com/event/228732/
    ///     let query = QueryBuilder::begin().event_id(228732).build();
    ///     if let Ok(query) = query {
    ///         let client = ConnpassClient::new();
    ///         let res = client.send_request(query).await;
    ///         match res {
    ///             Ok(r) => println!("{:?}", r),
    ///             Err(err) => eprintln!("{:?}", err),
    ///         }
    ///     }
    /// }
    /// ```
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

/// The client using blokcing. This one capitalizes on `reqwest::blocking` API.
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

        /// Sends requests and gets response from API in the blocking context.
        /// The response is internally converted to `response::ConnpassResponse` with handling errors.
        ///
        /// # Arguments
        /// If no condition is set to `query` and it's passed, the default options are applied.
        /// The defaults are described in the connpass API specification page.
        ///
        /// # Example:
        /// ```
        /// use connpass_rs::{client::blocking::ConnpassClient, query::builder::QueryBuilder};
        ///
        /// fn main() {
        ///     // fetch https://rust.connpass.com/event/228732/
        ///     let query = QueryBuilder::begin().event_id(228732).build();
        ///     if let Ok(query) = query {
        ///         let client = ConnpassClient::new();
        ///         let res = client.send_request(query);
        ///         match res {
        ///             Ok(r) => println!("{:?}", r),
        ///             Err(err) => eprintln!("{:?}", err),
        ///         }
        ///     }
        /// }
        /// ```
        #[allow(clippy::needless_doctest_main)]
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
