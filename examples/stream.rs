extern crate log;
extern crate pretty_env_logger;

use std::path::PathBuf;

use bracket::{
    registry::Registry,
    template::{Loader, Templates},
    Result,
};

use serde_json::json;

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    let content = include_str!("files/whitespace.md");
    let data = json!({
        "foo": "bar",
    });

    let registry = Registry::new();
    registry.stream("stream.rs", content, &data)?;
    Ok(())
}