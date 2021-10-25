# connpass-rs

An API client for connpass.com. The specification is [here](https://connpass.com/about/api/).

## Example

### Non-blocking client

The asynchronous example uses tokio and reqwest crate, so the Cargo.toml settings could look like the below:

```
[dependencies]
tokio = { version = "1.12.0", features = ["full"] }
connpass-rs = "0.1.0"
```

The code is like (the complete example in [here](examples/get_single_event.rs)):

```rust
use connpass_rs::{client::ConnpassClient, query::builder::QueryBuilder};

#[tokio::main]
async fn main() {
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
```

### Blocking client

There's an optional "blocking" client that can be enabled:

```
[dependencies]
connpass-rs = { version = "0.1.0", features = ["blocking"] }
```

And then, the code looks like (the complete example is [here](examples/get_single_event_blocking.rs)):

```rust
use connpass_rs::{client::blocking::ConnpassClient, query::builder::QueryBuilder};

fn main() {
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
```

## License

MIT
