use connpass_rs::{client::ConnpassClient, query::builder::QueryBuilder};

#[tokio::main]
async fn main() {
    // fetch https://rust.connpass.com/event/228732/
    let query = QueryBuilder::begin().event_id(228732).build();
    if let Ok(query) = query {
        let client = ConnpassClient::new();
        let res = client.send_request(query).await;
        match res {
            Ok(r) => println!("{:?}", r),
            Err(err) => eprintln!("{:?}", err),
        }
    }
}
