use once_cell::unsync::Lazy;
use reqwest::{header::USER_AGENT, Client};

use crate::{query::Query, response::ConnpassResponse};

const BASE_URL: &'static str = "https://connpass.com/api/v1/event/";
const CRATE_USER_AGENT: Lazy<String> = Lazy::new(|| {
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

    pub async fn send_request(self, query: Query) -> Result<ConnpassResponse, reqwest::Error> {
        self.client
            .get(BASE_URL)
            .header(USER_AGENT, CRATE_USER_AGENT.as_str())
            .query(&query.to_reqwest_query())
            .send()
            .await?
            .json::<ConnpassResponse>()
            .await
    }
}

#[cfg(test)]
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
