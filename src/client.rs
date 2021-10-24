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

    pub fn with_client(client: Client) -> Self {
        ConnpassClient { client }
    }

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

#[cfg(ignore)]
mod test {
    use crate::query::builder::QueryBuilder;

    use super::ConnpassClient;

    #[tokio::test]
    async fn test() {
        let query = QueryBuilder::begin()
            .keywords(vec![
                "python".to_string(),
                "rust".to_string(),
                "swift".to_string(),
            ])
            .build()
            .unwrap();
        let client = ConnpassClient::new();
        let task = client.clone().send_request(query);
        let r = task.await;
        println!("{:?}", r.unwrap());
    }
}
