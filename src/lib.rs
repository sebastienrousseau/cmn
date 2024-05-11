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
//!| -- | -- | -- |
//!| APERY | Apéry's constant, is the sum of the reciprocals of the positive cubes.<br>`ζ(3) ≈ 1.2020569032` | Used in various mathematical calculations and series approximations. |
//!| AVOGADRO | Avogadro's constant is the number of atoms or molecules in one mole of a substance.<br>`N_A ≈ 6.02214076 x 10^23 mol^-1` | The number of atoms in 12 grams of carbon-12 is 6.02214076 × 10^23. This can be used to calculate the number of atoms or molecules in a given sample. |
//!| BOLTZMANN | Boltzmann's constant is the physical constant relating the temperature of a system to the average kinetic energy of its constituent particles.<br>`k_B ≈ 1.380648 x 10^-23 J K^-1` | The kinetic energy of an atom at room temperature is about 2.0 × 10^-21 joules. This can be used to calculate the temperature of a system, or to calculate the average kinetic energy of its particles. |
//!| CATALAN | Catalan's constant, is the sum of the alternating harmonic series.<br>`C ≈ 0.915965594177219` | Used in various mathematical calculations and series approximations. |
//!| COULOMB | Coulomb's constant, is the proportionality constant in Coulomb's law.<br>`k_e ≈ 8.9875517923` x 10^9 N m^2 C^-2 | Used in calculations related to electrostatic forces and electric fields. |
//!| EULER | Euler's constant is a mathematical constant approximately equal to 2.71828.<br>`e ≈ 2.7182818284590452353602874713527` | The sum of the infinite series 1 + 1/2 + 1/3 + ... is equal to Euler's constant, e. This can be used to calculate the sum of an infinite series, or to calculate the logarithm of a number. |
//!| FARADAY | Faraday constant, which represents the amount of electric charge carried by one mole of electrons.<br>`F ≈ 96485.33212 C mol^-1` | Used in calculations related to electrochemistry and electrolysis. |
//!| GAMMA | The gamma constant is a mathematical constant approximately equal to 0.57721.<br>`γ ≈ 0.5772156649015329` | The gamma function of 2 is equal to 1. This can be used to calculate the gamma function of a number, or to calculate the factorial of a number. |
//!| GAS_CONSTANT | The gas constant, which relates the energy scale to the temperature scale in the ideal gas law.<br>`R ≈ 8.314462618 J mol^-1 K^-1` | Used in calculations related to the behavior of gases and thermodynamics. |
//!| GLAISHER_KINKELIN | Glaisher-Kinkelin constant, which arises in the asymptotic expansion of the Barnes G-function.<br>`A ≈ 1.2824271291` | Used in various mathematical calculations and series approximations. |
//!| GRAVITATIONAL_CONSTANT | The gravitational constant, is the proportionality constant in Newton's law of universal gravitation.<br>`G ≈ 6.67430 x 10^-11 m^3 kg^-1 s^-2` | Used in calculations related to gravitational forces and celestial mechanics. |
//!| HASH_ALGORITHM | The hash algorithm used to generate the hash. The default is Blake3. | The hash of the string "Hello, world!" is 5eb63bbbe01eeed093cb22bb8f5acdc32790160b123138d53f2173b8d3dc3eee. This can be used to verify the integrity of data, or to create a unique identifier for a file. |
//!| HASH_COST | The cost of the hash. | The hash cost of Blake3 is<br>`2^128`. This can be used to determine how secure a hash algorithm is. |
//!| HASH_LENGTH | The length of the hash. | The hash length of Blake3 is 32 bytes. This can be used to determine how much space is required to store the hash output. |
//!| KHINCHIN | Khinchin's constant, which appears in the theory of continued fractions.<br>`K ≈ 2.6854520010` | Used in various mathematical calculations and series approximations. |
//!| PHI | The golden ratio is a number approximately equal to 1.618033988749895.<br>`φ = (1 + √5) / 2 ≈ 1.6180339887498948482045868343656` | The golden ratio can be used to create a symmetrical design, or a design that is pleasing to the eye. |
//!| Pi (π) | Pi is the ratio of a circle's circumference to its diameter.<br>`π ≈ 3.14159265358979323846264338327950288` | The circumference of a circle with a radius of 1 is equal to pi. This can be used to calculate the circumference, area, and volume of circles, spheres, and other geometric shapes. |
//!| PLANCK | Planck's constant, which relates the energy of a photon to its frequency.<br>`h ≈ 6.62607015 x 10^-34 J s` | The energy of a photon of light with a wavelength of 500 nanometers is equal to Planck's constant multiplied by the frequency of the light. This can be used to calculate the energy of photons and other elementary particles. |
//!| PLANCK_REDUCED | Planck's reduced constant, is Planck's constant divided by 2π.<br>`ħ = h / (2π) ≈ 1.054571817 x 10^-34 J s` | Used in quantum mechanics and related calculations. |
//!| SILVER_RATIO | The silver ratio is one of the silver means.<br>`δ_s = 1 + √2 ≈ 2.4142135623730950488016887242097` | The silver ratio can be used to create a symmetrical design, or a design that is pleasing to the eye. |
//!| SPEED_OF_LIGHT | The speed of light in vacuum.<br>`c ≈ 299792458 m s^-1` | Used in calculations related to relativity and electromagnetic phenomena. |
//!| SPECIAL_CHARS | A set of special characters. | The special characters are: !@#$%^&*()_+-={}[]|\:;"'<>,.?` |
//!| SQRT2 | The square root of 2.<br>`√2 ≈ 1.4142135623730950488016887242097` | The area of a circle with a radius of 1 is equal to the square root of 2. This can be used to calculate the area and volume of circles, spheres, and other geometric shapes. |
//!| SQRT3 | The square root of 3.<br>`√3 ≈ 1.7320508075688772935274463415059` | The area of a circle with a radius of 1 is equal to the square root of 3. This can be used to calculate the area and volume of circles. |
//!| SQRT5 | The square root of 5.<br>`√5 ≈ 2.23606797749979` | The area of a circle with a radius of 1 is equal to the square root of 5. |
//!| TAU | The circle constant, is the ratio of a circle's circumference to its radius.<br>`τ = 2π ≈ 6.28318530717958647692528676655900577` | The circumference of a circle with a radius of 1 is equal to tau. |
//!| VACUUM_PERMEABILITY | The vacuum permeability, which relates magnetic induction to magnetic field strength.<br>`μ_0 ≈ 1.25663706212 x 10^-6 N A^-2` | Used in calculations related to electromagnetism and magnetic fields. |
//!| VACUUM_PERMITTIVITY | The vacuum permittivity, which relates electric displacement to electric field strength.<br>`ε_0 ≈ 8.8541878128 x 10^-12 F m^-1` | Used in calculations related to electromagnetism and electric fields. |
//!
//! The following table lists the dictionaries available in the Common library.
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
//! use cmn::words::WORD_LIST;
//!
//! // Constants
//! let constants = Constants::new();
//! let constant = constants.constant("EULER");
//! assert_eq!(constant.unwrap().name, "EULER");
//!
//! // Words
//! let words = Words::default();
//! let words_list = words.words_list();
//! // Checking the first three elements to verify correct initialization and ordering
//! assert_eq!(words_list.len(), WORD_LIST.len(), "Default words list length should match WORD_LIST length.");
//! assert_eq!(words_list[0], "aboard", "Check that words list is sorted and starts with the first word.");
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

use std::collections::HashSet;

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
        let words_data = self
            .fields
            .get("words")
            .map(|words_array| {
                words_array
                    .as_array()
                    .expect("Words data is not an array")
                    .iter()
                    .map(|word_value| {
                        word_value.as_str().unwrap().to_string()
                    })
                    .collect::<Vec<String>>()
            })
            .unwrap_or_default(); // Add this line

        Words {
            words: HashSet::from_iter(words_data),
        }
    }
    /// Parses a string of JSON data and returns a new instance of the `Common` structure.
    pub fn parse(input: &str) -> Result<Self, serde_json::Error> {
        match serde_json::from_str::<Common>(input) {
            Ok(common) => {
                if common.fields.is_null()
                    || common.fields.is_object()
                        && common.fields.as_object().unwrap().is_empty()
                {
                    // If the `fields` object is null or empty, return the default `Common` instance
                    Ok(Common::default())
                } else {
                    Ok(common)
                }
            }
            Err(e) => {
                // Handle the JSON parsing error
                match e.classify() {
                    serde_json::error::Category::Io => {
                        eprintln!("I/O error occurred: {}", e);
                    }
                    serde_json::error::Category::Syntax => {
                        eprintln!("JSON syntax error: {}", e);
                    }
                    serde_json::error::Category::Data => {
                        eprintln!("Invalid JSON data: {}", e);
                    }
                    serde_json::error::Category::Eof => {
                        eprintln!("Unexpected end of JSON input");
                    }
                }
                Err(e)
            }
        }
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
