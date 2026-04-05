#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Common (CMN)
//!
//! 121 mathematical and cryptographic constants for Rust.
//! Zero runtime cost. `no_std` compatible.
//!
//! ## Why CMN?
//!
//! `std::f64::consts` provides 11 mathematical constants. CMN
//! extends that with 44 physical, cryptographic, and series
//! constants — plus a runtime lookup API, 14 utility macros,
//! and a built-in word-list module. Every constant resolves at
//! compile time. Works in `no_std` environments.
//!
//! ## Modules
//!
//! - **[`constants`]** — 121 `const` values (PI, Avogadro,
//!   Planck, etc.). With `std`: runtime `Constants` lookup API
//!   + `ConstantValue` typed enum.
//! - **[`macros`]** — 14 utility macros for min/max,
//!   range-checks, collections, and parsing.
//! - With `std` feature: **[`words`]** module, **[`datetime`]**
//!   utilities, and **`Common`** JSON bridge.
//!
//! ## Quick Start
//!
//! ```rust
//! // Compile-time constants — always available, even in no_std
//! use cmn::constants::{PI, EULER, SPEED_OF_LIGHT};
//!
//! assert_eq!(PI, core::f64::consts::PI);
//! ```
//!
//! ```rust
//! // Runtime lookup — requires the `std` feature (default)
//! use cmn::Constants;
//! use cmn::Words;
//! use cmn::words::WORD_LIST;
//!
//! let constants = Constants::new();
//! let euler = constants.constant("EULER").unwrap();
//! assert_eq!(euler.name, "EULER");
//!
//! let words = Words::default();
//! let list = words.words_list();
//! assert_eq!(list.len(), WORD_LIST.len());
//! assert_eq!(list[0], "aboard");
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | Default | Enables |
//! |---------|---------|---------|
//! | `std`   | Yes     | `Constants` struct, `Words`, `Common`, serde support |
//!
//! For `no_std`, disable default features:
//! ```toml
//! [dependencies]
//! cmn = { version = "0.0.6", default-features = false }
//! ```
//!
//! ## License
//!
//! Dual-licensed under
//! [Apache 2.0](https://opensource.org/licenses/Apache-2.0) or
//! [MIT](https://opensource.org/licenses/MIT), at your option.
//!
#![doc(
    html_favicon_url = "https://cloudcdn.pro/cloudcdn/v1/logos/kura.svg",
    html_logo_url = "https://cloudcdn.pro/cloudcdn/v1/logos/kura.svg",
    html_root_url = "https://docs.rs/cmn"
)]
#![crate_name = "cmn"]
#![crate_type = "lib"]

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// 14 utility macros for assertions, collections, range-checks,
/// and string operations. Available in `no_std`.
pub mod macros;

/// 121 mathematical, physical, and cryptographic constants as
/// compile-time `const` values. The `const` values are always
/// available, even in `no_std`. The runtime `Constants` lookup
/// API and `ConstantValue` enum require the `std` feature.
pub mod constants;
#[cfg(feature = "std")]
pub use constants::Constants;

/// A word-list module for passphrase generation and text
/// processing. Backed by `HashSet<String>` for O(1) lookups
/// with a curated built-in `WORD_LIST`. Requires `std`.
#[cfg(feature = "std")]
pub mod words;
#[cfg(feature = "std")]
pub use words::Words;

/// Lightweight datetime utilities for ISO 8601 parsing,
/// relative time formatting, and duration calculations.
/// No external datetime crate required. Requires `std`.
#[cfg(feature = "std")]
pub mod datetime;

/// The `Common` structure provides a central location to store
/// data that is commonly used throughout the library. Requires
/// the `std` feature for serde JSON support.
///
/// # Fields
///
/// * `constants`: A reference to the `Constants` structure.
/// * `words`: A reference to the `Words` structure.
#[cfg(feature = "std")]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Common {
    #[serde(flatten)]
    fields: serde_json::Value,
}

#[cfg(feature = "std")]
impl Common {
    /// Creates a new instance of the `Common` structure.
    pub fn new() -> Self {
        Self {
            fields: serde_json::Value::Null,
        }
    }
    /// Returns the `Constants` instance.
    pub fn constants(&self) -> Constants {
        Constants {
            ..Default::default()
        }
    }
    /// Returns a new instance of the `Words` structure.
    ///
    /// Gracefully handles malformed JSON — returns an empty
    /// `Words` if the `words` field is missing, not an array,
    /// or contains non-string elements.
    pub fn words(&self) -> Words {
        let words_data: Vec<String> = self
            .fields
            .get("words")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        Words {
            words: words_data.into_iter().collect(),
        }
    }

    /// Parses a string of JSON data and returns a new
    /// instance of the `Common` structure.
    pub fn parse(input: &str) -> Result<Self, serde_json::Error> {
        let common = serde_json::from_str::<Common>(input)?;
        let is_empty = common.fields.is_null()
            || common
                .fields
                .as_object()
                .is_some_and(|obj| obj.is_empty());
        if is_empty {
            Ok(Common::default())
        } else {
            Ok(common)
        }
    }
}

#[cfg(feature = "std")]
impl Default for Common {
    fn default() -> Self {
        Self::new()
    }
}

/// Main entry point for the `Common (CMN)` library.
/// Requires `std`.
#[cfg(feature = "std")]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("CMN_TEST_MODE").unwrap_or_default() == "1" {
        return Err("Simulated error".into());
    }
    let name = "cmn";
    println!("Welcome to `{}` 👋!", { name }.to_uppercase());
    println!(
        "A Rust library for accessing a collection of \
         mathematical and cryptographic constants."
    );
    Ok(())
}
