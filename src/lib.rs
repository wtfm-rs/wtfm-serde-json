#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! # WTFM for [serde_json]
//!
//! Although this is an empty crate with doc-tests only, `cargo doc`
//! by default builds documentation for dependencies
//!
//! ```sh
//! % du -hsc docs/doc/*
//! 4.0K	docs/doc/crates.js
//! 4.0K	docs/doc/help.html
//! 64K	docs/doc/itoa
//! 2.9M	docs/doc/memchr
//! 696K	docs/doc/search.index
//! 4.4M	docs/doc/serde_core
//! 4.1M	docs/doc/serde_json
//! 4.0K	docs/doc/settings.html
//! 3.8M	docs/doc/src
//! 4.0K	docs/doc/src-files.js
//! 1.8M	docs/doc/static.files
//! 548K	docs/doc/trait.impl
//! 248K	docs/doc/type.impl
//!  16K	docs/doc/wtfm_serde_json
//!  52K	docs/doc/zmij
//!  19M	total
//! ```
//! We can navigate to them via the sidebar menu.
//!
//! ## It works!
//! ```
//! use serde_json::{Result, Value};
//!
//! fn main() -> Result<()> {
//!    // Some JSON input data as a &str. Maybe this comes from the user.
//!    let data = r#"
//!        {
//!            "name": "John Doe",
//!            "age": 43,
//!            "phones": [
//!                "+44 1234567",
//!                "+44 2345678"
//!            ]
//!        }"#;
//!
//!    // Parse the string of data into serde_json::Value.
//!    let v: Value = serde_json::from_str(data)?;
//!
//!    // Access parts of the data by indexing with square brackets.
//!    debug_assert_eq!(
//!        format!("Please call {} at the number {}", v["name"], v["phones"][0]),
//!        "Please call \"John Doe\" at the number \"+44 1234567\""
//!    );
//!
//!    Ok(())
//! }
//! ```
