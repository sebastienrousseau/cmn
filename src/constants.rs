// Copyright © 2023 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};

/// Contains several commonly used mathematical and cryptographic constants.
#[non_exhaustive]
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct Constant {
    /// The name of the constant.
    pub name: &'static str,

    /// The value of the constant.
    pub value: String,
}

/// The `Constants` structure holds mathematical and hash constants.
#[derive(Clone, Serialize, Debug)]
pub struct Constants {
    /// A vector of constants.
    pub constants: Vec<Constant>,
}

impl Constants {
    /// Returns a vector of tuples with the constant name and its value.
    ///
    /// # Arguments
    /// * `name` - The name of the constant.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate cmn;
    /// use cmn::constants::Constants;
    ///
    /// let constants = Constants::new();
    /// let constant = constants.constant("EULER");
    ///
    /// assert_eq!(constant.unwrap().name, "EULER");
    ///
    /// ```
    ///
    pub fn constant(&self, name: &str) -> Option<Constant> {
        self.constants
            .iter()
            .find(|constant| constant.name == name)
            .cloned()
    }

    /// Returns a vector of tuples with the constant name and its value.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate cmn;
    /// use cmn::constants::Constants;
    ///
    /// let constants = Constants::new();
    /// assert!(constants.constants().len() >= 9);
    ///
    /// ```
    pub fn constants(&self) -> &Vec<Constant> {
        &self.constants
    }

    /// Create a new instance of the `Constants` structure.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate cmn;
    /// use cmn::constants::Constants;
    ///
    /// let constants = Constants::new();
    /// assert_eq!(constants.constants().len(), 16);
    ///
    /// ```
    ///
    pub fn new() -> Self {
        let constants = vec![
            Constant {
                name: "AVOGADRO",
                value: AVOGADRO.to_string(),
            },
            Constant {
                name: "BOLTZMANN",
                value: BOLTZMANN.to_string(),
            },
            Constant {
                name: "EULER",
                value: EULER.to_string(),
            },
            Constant {
                name: "GAMMA",
                value: GAMMA.to_string(),
            },
            Constant {
                name: "HASH_ALGORITHM",
                value: HASH_ALGORITHM.to_string(),
            },
            Constant {
                name: "HASH_COST",
                value: HASH_COST.to_string(),
            },
            Constant {
                name: "HASH_LENGTH",
                value: HASH_LENGTH.to_string(),
            },
            Constant {
                name: "PHI",
                value: PHI.to_string(),
            },
            Constant {
                name: "PI",
                value: PI.to_string(),
            },
            Constant {
                name: "PLANCK",
                value: PLANCK.to_string(),
            },
            Constant {
                name: "SILVER_RATIO",
                value: SILVER_RATIO.to_string(),
            },
            Constant {
                name: "SPECIAL_CHARS",
                value: SPECIAL_CHARS.iter().collect::<String>(),
            },
            Constant {
                name: "SQRT2",
                value: SQRT2.to_string(),
            },
            Constant {
                name: "SQRT3",
                value: SQRT3.to_string(),
            },
            Constant {
                name: "SQRT5",
                value: SQRT5.to_string(),
            },
            Constant {
                name: "TAU",
                value: TAU.to_string(),
            },
        ];

        Self { constants }
    }

    /// Returns `true` if the `Constants` structure is valid.
    /// Otherwise, returns `false`.
    pub fn is_valid(&self) -> bool {
        self.constants().iter().all(|constant| {
            !constant.name.is_empty() && !constant.value.is_empty()
        })
    }
}

impl Default for Constants {
    fn default() -> Self {
        Self::new()
    }
}

/// Enum to represent the different constant values.
#[derive(Debug, Clone, Serialize)]
pub enum ConstantValue {
    /// A float value represented as `f64`.
    Float(f64),
    /// A string value.
    String(String),
    /// An unsigned 32-bit integer value represented as `u32`.
    U32(u32),
    /// An unsigned integer with the size of a pointer represented
    /// as `usize`.
    Usize(usize),
    /// An array of characters represented as `&'static [char]`.
    CharArray(&'static [char]),
}

/// Avogadro's constant
/// Approximately 6.02214076 x 10^23
pub const AVOGADRO: f64 = 602214076000000000000000.0;

/// Boltzmann's constant
/// Approximately 1.380648 x 10^-23
pub const BOLTZMANN: f64 = 1.380648e-23;

/// The base of the natural logarithm, Euler's number (e).
/// e ≈ 2.7182818284590452353602874713527
pub const EULER: f64 = std::f64::consts::E;

/// The mathematical constant `γ` or the Euler–Mascheroni constant. It
/// is the limit of the difference between the harmonic series and the
/// natural logarithm of the natural numbers.
pub const GAMMA: f64 = 0.5772156649015329;

/// The hash algorithm used. The default is Blake3.
pub const HASH_ALGORITHM: &str = "Blake3";

/// The cost of the hash algorithm. The default is 8.
pub const HASH_COST: u32 = 8;

/// The hash length is the length of the hash in bytes.
/// - The default is 32.
/// - The maximum is 64.
/// - The minimum is 16.
pub const HASH_LENGTH: usize = 32;

/// The mathematical constant `φ` or the golden ratio. It is the
/// limit of the ratio of consecutive Fibonacci numbers.
/// Φ = (1+√5)/2 = 2.cos(π/5). Diagonal of a unit-side pentagon.
pub const PHI: f64 = (1.0 + SQRT5) / 2.0;

/// The mathematical constant `π` or the ratio of a circle's
/// circumference to its diameter.
pub const PI: f64 = std::f64::consts::PI;

/// The Planck constant, `h`.
pub const PLANCK: f64 = 6.62607015e-34;

/// The mathematical constant `δs' or the silver ratio (or silver mean).
/// δs = 1+√2. One of the silver means (n+sqrt(n2+1))/2 for n>0.
pub const SILVER_RATIO: f64 = 1.0 + SQRT2;

/// A set of special characters.
pub const SPECIAL_CHARS: &[char] = &[
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '=',
    '[', ']', '{', '}', '|', ';', ':', '"', '<', '>', ',', '.', '?',
    '/', '~', '`',
];

/// The mathematical constant `√2` or the Pythagora's constant or the
/// square root of 2. It is the diagonal of a square with unit side
/// length.
/// 2 = 1 + √2
pub const SQRT2: f64 = std::f64::consts::SQRT_2;

/// The mathematical constant `√3` or the  principal square root of 3.
/// It is the length of the side of an equilateral triangle with unit
/// side length.
/// 3 = 1 + √3
pub const SQRT3: f64 = 1.732_050_807_568_877_2;

/// The mathematical constant `√5` or the principal square root of 5.
/// It is the length of the diagonal of a regular pentagon with unit
/// side length.
/// 5 = 2 + 2√5
pub const SQRT5: f64 = 2.236_067_977_499_79;

/// The mathematical constant `τ` or the ratio of a circle's
/// circumference to its radius.
pub const TAU: f64 = std::f64::consts::TAU;
