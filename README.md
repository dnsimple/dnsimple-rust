# DNSimple Rust Client

A Rust client for the [DNSimple API v2](https://developer.dnsimple.com/v2/).

[![Build Status](https://github.com/dnsimple/dnsimple-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/dnsimple/dnsimple-rust/actions/workflows/ci.yml)

## Documentation

- [dnsimple-rust crates.io](https://crates.io/crates/dnsimple)
- [DNSimple API documentation](https://developer.dnsimple.com/)
- [DNSimple API examples repository](https://github.com/dnsimple/dnsimple-api-examples)
- [DNSimple support documentation](https://support.dnsimple.com/)

## Requirements

- Rust: 1.83+

## Usage

```rust
use dnsimple::dnsimple::{Client, new_client};

let client = new_client(false, String::from("AUTH_TOKEN"));
let identity_response = client.identity().whoami().unwrap().data.unwrap();
```

### Sandbox Environment

We highly recommend testing against our [sandbox environment](https://developer.dnsimple.com/sandbox/) before using our
production environment. This will allow you to avoid real purchases, live charges on your credit card, and reduce the
chance of your running up against rate limits.

The client supports both the production and sandbox environment. To switch to sandbox pass the sandbox API host setting the
`sandbox` option to `true` when you construct the client:

```rust
use dnsimple::dnsimple::{Client, new_client};

let client = new_client(true, String::from("AUTH_TOKEN"));
let identity_response = client.identity().whoami().unwrap().data.unwrap();
```

You will need to ensure that you are using an access token created in the sandbox environment.
Production tokens will *not* work in the sandbox environment.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for details.

## License

Copyright (c) 2015-2024 DNSimple Corporation. This is Free Software distributed under the MIT license.
