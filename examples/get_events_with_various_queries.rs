use connpass_rs::{
    client::ConnpassClient,
    query::{builder::QueryBuilder, types::OrderOption},
};

#[tokio::main]
async fn main() {
    let query = QueryBuilder::begin()
        .keyword_or("Python".to_string())
        .keyword_or("機械学習".to_string())
        .yms(vec![202110, 202111])
        .order(OrderOption::Newer)
        .count(15)
        .build();
    if let Ok(query) = query {
        let client = ConnpassClient::new();
        let res = client.send_request(query).await;
        match res {
            Ok(r) => println!("{:?}", r),
            Err(err) => eprintln!("{:?}", err),
        }
    }
}
