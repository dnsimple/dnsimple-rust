# DNSimple Rust Client

A Rust client for the [DNSimple API v2](https://developer.dnsimple.com/v2/).

[![Build Status](https://github.com/dnsimple/dnsimple-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/dnsimple/dnsimple-rust/actions/workflows/ci.yml)


## :warning: Development Warning

This project targets the development of the API client for the [DNSimple API v2](https://developer.dnsimple.com/v2/).

This version is currently under development, therefore the methods and the implementation should he considered a 
work-in-progress. Changes in the method naming, method signatures, public or internal APIs may happen at any time.

The code is tested with an automated test suite connected to a continuous integration tool, therefore you should not 
expect :bomb: bugs to be merged into main. Regardless, use this library at your own risk. :boom:


## Usage

```
use dnsimple::dnsimple::{Client, new_client};

let client = new_client(true, String::from("AUTH_TOKEN"));
let identity_response = client.identity().whoami().unwrap().data.unwrap();
```

## License

Copyright (c) 2015-2021 DNSimple Corporation. This is Free Software distributed under the MIT license.
