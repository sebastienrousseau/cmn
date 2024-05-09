// Copyright © 2023 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//!
//! A Rust library for accessing a collection of mathematical and
//! cryptographic constants
//!
//! *Part of the [Mini Functions][0] family of libraries.*
//!
//! [![Common (CMN)](https://kura.pro/cmn/images/titles/title-cmn.svg)](https://cmnlib.com/cmn)
//!
//! <center>
//!
//! [![Crates.io](https://img.shields.io/crates/v/cmn.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/cmn)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/cmn)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.4-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/cmn)
//! [![License](https://img.shields.io/crates/l/cmn.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//!
//! </center>
//!
//! ## Overview 📖
//!
//! `Common (CMN)` is a modern, fast, and user-friendly library that makes it easy to access a wide range of mathematical and cryptographic constants.
//!
//! ## Features ✨
//!
//! The `Common (CMN)` uses the `serde` crate to serialize and deserialize the data.
//!
//! The library has three modules:
//!
//! - **Macros**: This module contains functions for generating macros that can be used to access the constants.
//! - **Constants**: This module contains the Constants structure, which provides a collection of constant values that are used throughout the library.
//! - **Words**: This module contains the Words structure, which provides a collection of words that are used throughout the library.
//!
//! ### Mathematical and Cryptographic Constants
//!
//! The following table lists the most important mathematical and cryptographic constants available in the `Common (CMN)` library:
//!
//!| Constants | Description | Example |
//!| --- | --- | --- |
//!| AVOGADRO | Avogadro's constant is the number of atoms or molecules in one mole of a substance. | The number of atoms in 12 grams of carbon-12 is 6.02214076 × 10^23. This can be used to calculate the number of atoms or molecules in a given sample. |
//!| BOLTZMANN | Boltzmann's constant is the physical constant relating the temperature of a system to the average kinetic energy of its constituent particles. | The kinetic energy of an atom at room temperature is about 2.0 × 10^-21 joules. This can be used to calculate the temperature of a system, or to calculate the average kinetic energy of its particles. |
//!| EULER | Euler's constant is a mathematical constant approximately equal to 2.71828. | The sum of the infinite series 1 + 1/2 + 1/3 + ... is equal to Euler's constant, e. This can be used to calculate the sum of an infinite series, or to calculate the logarithm of a number. |
//!| GAMMA | The gamma constant is a mathematical constant approximately equal to 0.57721. | The gamma function of 2 is equal to 1. This can be used to calculate the gamma function of a number, or to calculate the factorial of a number. |
//!| HASH_ALGORITHM | The hash algorithm used to generate the hash. The default is Blake3. | The hash of the string "Hello, world!" is 5eb63bbbe01eeed093cb22bb8f5acdc32790160b123138d53f2173b8d3dc3eee. This can be used to verify the integrity of data, or to create a unique identifier for a file. |
//!| HASH_COST | The cost of the hash. | The hash cost of Blake3 is 2^128. This can be used to determine how secure a hash algorithm is. |
//!| HASH_LENGTH | The length of the hash. | The hash length of Blake3 is 32 bytes. This can be used to determine how much space is required to store the hash output. |
//!| PHI | The golden ratio is a number approximately equal to 1.618033988749895. | The golden ratio can be used to create a symmetrical design, or a design that is pleasing to the eye. |
//!| PI | Pi is the ratio of a circle's circumference to its diameter. | The circumference of a circle with a radius of 1 is equal to pi. This can be used to calculate the circumference, area, and volume of circles, spheres, and other geometric shapes. |
//!| PLANCK | Planck's constant is a physical constant that is approximately equal to 6.62607015 × 10^−34 joule seconds. | The energy of a photon of light with a wavelength of 500 nanometers is equal to Planck's constant multiplied by the frequency of the light. This can be used to calculate the energy of photons and other elementary particles. |
//!| SILVER_RATIO | The silver ratio is a number approximately equal to 1.414213562373095. | The silver ratio can be used to create a symmetrical design, or a design that is pleasing to the eye. |
//!| SPECIAL_CHARS | A list of special characters. | The special characters are: !@#$%^&*()_+-={}[]|\:;"'<>,.? |
//!| SQRT2 | The square root of 2 is a number approximately equal to 1.414213562373095. | The area of a circle with a radius of 1 is equal to the square root of 2. This can be used to calculate the area and volume of circles, spheres, and other geometric shapes. |
//!| SQRT3 | The square root of 3 is a number approximately equal to 1.732050807568877. | The area of a circle with a radius of 1 is equal to the square root of 3. This can be used to calculate the area and volume of circles
//!| SQRT5 | The square root of 5 is a number approximately equal to 2.23606797749979. | The area of a circle with a radius of 1 is equal to the square root of 5. |
//!| TAU | Tau is a number approximately equal to 6.283185307179586. | The circumference of a circle with a radius of 1 is equal to tau. |
//!
//! The following table lists the dictionaries available in the Common
//! library.
//!
//!| Words | Description |
//!| --- | --- |
//!| `words` | Contains a dictionary of common words. |
//!
//! ## Usage
//!
//! Common can be any `serde::Serialize` or `serde::Deserialize` types
//!
//! ## Examples
//!
//! ```rust
//!
//! // Import the Common libraries
//! extern crate cmn;
//! use cmn::Constants;
//! use cmn::Words;
//!
//! // Constants
//! let constants = Constants::new();
//! let constant = constants.constant("EULER");
//! assert_eq!(constant.unwrap().name, "EULER");
//!
//! // Words
//! let words = Words::new();
//! let words_list = words.words_list();
//! assert_eq!(words_list[0], "aboard");
//!
//! ```
//! ## License
//! The project is licensed under the terms of both the MIT license and
//! the Apache License (Version 2.0).
//! - [Apache License, Version 2.0](https://opensource.org/licenses/Apache-2.0)
//! - [MIT license](https://opensource.org/licenses/MIT)
//!
//! ## Contribution
//! Unless you explicitly state otherwise, any contribution
//! intentionally submitted for inclusion in the work by you, as
//! defined in the Apache-2.0 license, shall be dual licensed as above,
//! without any additional terms or conditions.
//!
//! [0]: https://cmnlib.com/ "MiniFunctions"
//!
#![cfg_attr(feature = "bench", feature(test))]
#![doc(
    html_favicon_url = "https://kura.pro/cmn/images/favicon.ico",
    html_logo_url = "https://kura.pro/cmn/images/logos/cmn.svg",
    html_root_url = "https://docs.rs/cmn"
)]
#![crate_name = "cmn"]
#![crate_type = "lib"]

/// The `serde` crate provides the `Serialize` and `Deserialize` traits
/// that are used to serialize and deserialize the data.
use serde::{Deserialize, Serialize};

/// The `macros` module contains functions for generating macros.
pub mod macros;

/// The `constants` module contains the `Constants` structure, which
/// provides a collection of constant values that are used throughout
/// the library.
pub mod constants;
pub use constants::Constants;

/// The `words` module contains the `Words` structure, which provides a
/// collection of words that are used throughout the library.
pub mod words;
pub use words::Words;

/// The `Common` structure provides a central location to store data
/// that is commonly used throughout the library. The structure
/// implements the `Serialize` and `Deserialize` traits from the `serde`
/// crate to enable serialization and deserialization of the data.
///
/// # Fields
///
/// * `constants`: A reference to the `Constants` structure.
/// * `words`: A reference to the `Words` structure.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Common {
    #[serde(flatten)]
    fields: serde_json::Value,
}

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
    pub fn words(&self) -> Words {
        Words::new()
    }
    /// Parses a string of JSON data and returns a new instance of the
    /// `Common` structure.
    pub fn parse(
        input: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let common: Common = serde_json::from_str(input)?;
        Ok(common)
    }
}

impl Default for Common {
    /// Creates a new instance of the `Common` structure by calling
    /// `Self::new()`.
    fn default() -> Self {
        Self::new()
    }
}

/// This is the main entry point for the `Common (CMN)` library.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("CMN_TEST_MODE").unwrap_or_default() == "1" {
        return Err("Simulated error".into());
    }
    let name = "cmn";
    println!("Welcome to `{}` 👋!", { name }.to_uppercase());
    println!(
        "A Rust library for accessing a collection of mathematical and cryptographic constants."
    );
    Ok(())
}
