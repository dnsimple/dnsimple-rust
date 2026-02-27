# DNSimple Rust Client

A Rust client for the [DNSimple API v2](https://developer.dnsimple.com/v2/).

[![Build Status](https://github.com/dnsimple/dnsimple-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/dnsimple/dnsimple-rust/actions/workflows/ci.yml)

## Requirements

- Rust: 1.86+
- An activated DNSimple account

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dnsimple = "5.0"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Usage

This library uses async/await with the Tokio runtime:

```rust
use dnsimple::dnsimple::{Client, new_client};

#[tokio::main]
async fn main() {
    let client = new_client(false, String::from("AUTH_TOKEN"));
    let identity_response = client.identity().whoami().await.unwrap().data.unwrap();
}
```

## Configuration

### Sandbox Environment

We highly recommend testing against our [sandbox environment](https://developer.dnsimple.com/sandbox/) before using our
production environment. This will allow you to avoid real purchases, live charges on your credit card, and reduce the
chance of your running up against rate limits.

The client supports both the production and sandbox environment. To switch to sandbox pass the sandbox API host setting the
`sandbox` option to `true` when you construct the client:

```rust
use dnsimple::dnsimple::{Client, new_client};

#[tokio::main]
async fn main() {
    let client = new_client(true, String::from("AUTH_TOKEN"));
    let identity_response = client.identity().whoami().await.unwrap().data.unwrap();
}
```

You will need to ensure that you are using an access token created in the sandbox environment.
Production tokens will *not* work in the sandbox environment.

### Setting a custom `User-Agent` header

You can customize the `User-Agent` header for the calls made to the DNSimple API:

```rust
use dnsimple::dnsimple::{Client, new_client};

let mut client = new_client(false, String::from("AUTH_TOKEN"));
client.set_user_agent("my-app/1.0");
```

The value you provide will be prepended to the default `User-Agent` the client uses. For example, if you use `my-app/1.0`, the final header value will be `my-app/1.0 dnsimple-rust/0.1.0` (note that it will vary depending on the client version).

## Documentation

- [dnsimple-rust crates.io](https://crates.io/crates/dnsimple)
- [DNSimple API documentation](https://developer.dnsimple.com/)
- [DNSimple API examples repository](https://github.com/dnsimple/dnsimple-api-examples)
- [DNSimple support documentation](https://support.dnsimple.com/)

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for details.

## License

Copyright (c) 2021-2026 DNSimple Corporation. This is Free Software distributed under the [MIT License](LICENSE.txt).
