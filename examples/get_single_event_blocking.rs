//! Blocking API example.
//! If you use `connpass_rs::client::blocking::ConnpassClient`, then can send requests with blocking.
//! For example, this can be used for CLI tools with `clap` crate, etc.

use connpass_rs::{client::blocking::ConnpassClient, query::builder::QueryBuilder};

// This can run with `cargo run --example get_single_event_blocking --features blocking`.
fn main() {
    // fetch https://rust.connpass.com/event/228732/
    let query = QueryBuilder::begin().event_id(228732).build();
    if let Ok(query) = query {
        let client = ConnpassClient::new();
        let res = client.send_request(query);
        match res {
            Ok(r) => println!("{:?}", r),
            Err(err) => eprintln!("{:?}", err),
        }
    }
}
