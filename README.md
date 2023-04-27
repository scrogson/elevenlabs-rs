<div id="top"></div>

<p align="center">
    <a href="https://github.com/scrogson/elevenlabs-rs/stargazers">
        <img src="https://img.shields.io/github/stars/scrogson/elevenlabs-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/scrogson/elevenlabs-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/scrogson/elevenlabs-rs/ci?style=flat-square" alt="Build Status" />
    </a>
<a href="https://crates.io/crates/elevenlabs">
    <img src="https://img.shields.io/crates/d/elevenlabs?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/elevenlabs">
    <img src="https://img.shields.io/crates/v/elevenlabs?style=flat-square" alt="Crates.io" />
</a>

</p>

ElevenLabs client, generated from the OpenAPI spec.

# Usage

```rust
use elevenlabs::ElevenLabsClient;
use elevenlabs::model::*;

#[tokio::main]
async fn main() {
    let client = ElevenLabsClient::from_env();
    let response = client
        .get_generated_items_v1_history_get()
        .xi_api_key("your xi api key")
        .await
        .unwrap();
    println!("{:#?}", response);
}
```

This example loads configuration from environment variables, specifically:

* `ELEVEN_LABS_BASE_URL`



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
elevenlabs = "0.1.0"
```


# Documentation

* [Client Library Documentation](https://docs.rs/elevenlabs)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*
