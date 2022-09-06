# DNSimple Rust Client

A Rust client for the [DNSimple API v2](https://developer.dnsimple.com/v2/).

[![Build Status](https://github.com/dnsimple/dnsimple-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/dnsimple/dnsimple-rust/actions/workflows/ci.yml)


## Usage

```
use dnsimple::dnsimple::{Client, new_client};

let client = new_client(true, String::from("AUTH_TOKEN"));
let identity_response = client.identity().whoami().unwrap().data.unwrap();
```

## License

Copyright (c) 2015-2022 DNSimple Corporation. This is Free Software distributed under the MIT license.
