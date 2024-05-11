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
    /// Returns the value of the constant.
    ///
    /// # Example
    ///
    /// ```
    /// use cmn::constants::Constants;
    /// use cmn::constants::ConstantValue;
    ///
    /// let constants = Constants::new();
    /// let constant = constants.constant("EULER").unwrap();
    /// let value = constants.get_value(constant.name);
    ///
    /// if let Some(ConstantValue::Float(float_value)) = value {
    ///     assert!((float_value - 2.71828).abs() < 1e-5);
    /// }
    /// else {
    ///     panic!("Expected a float value");
    /// }
    ///
    /// ```
    pub fn get_value(&self, name: &str) -> Option<ConstantValue> {
        if let Some(constant) = self.constant(name) {
            match name {
                "HASH_COST" => {
                    constant.value.parse().map(ConstantValue::U32).ok()
                }
                "HASH_LENGTH" => constant
                    .value
                    .parse()
                    .map(ConstantValue::Usize)
                    .ok(),
                _ => {
                    if let Ok(float_value) =
                        constant.value.parse::<f64>()
                    {
                        Some(ConstantValue::Float(float_value))
                    } else if let Some(char_array) =
                        Self::get_char_array(name)
                    {
                        Some(ConstantValue::CharArray(char_array))
                    } else {
                        Some(ConstantValue::String(
                            constant.value.clone(),
                        ))
                    }
                }
            }
        } else {
            None
        }
    }

    fn get_char_array(name: &str) -> Option<&'static [char]> {
        match name {
            "SPECIAL_CHARS" => Some(SPECIAL_CHARS),
            _ => None,
        }
    }

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
    /// assert_eq!(constants.constants().len(), 28);
    ///
    /// ```
    ///
    pub fn new() -> Self {
        let constants = vec![
            Constant {
                name: "APERY",
                value: APERY.to_string(),
            },
            Constant {
                name: "AVOGADRO",
                value: AVOGADRO.to_string(),
            },
            Constant {
                name: "BOLTZMANN",
                value: BOLTZMANN.to_string(),
            },
            Constant {
                name: "CATALAN",
                value: CATALAN.to_string(),
            },
            Constant {
                name: "COULOMB",
                value: COULOMB.to_string(),
            },
            Constant {
                name: "EULER",
                value: EULER.to_string(),
            },
            Constant {
                name: "FARADAY",
                value: FARADAY.to_string(),
            },
            Constant {
                name: "GAMMA",
                value: GAMMA.to_string(),
            },
            Constant {
                name: "GAS_CONSTANT",
                value: GAS_CONSTANT.to_string(),
            },
            Constant {
                name: "GLAISHER_KINKELIN",
                value: GLAISHER_KINKELIN.to_string(),
            },
            Constant {
                name: "GRAVITATIONAL_CONSTANT",
                value: GRAVITATIONAL_CONSTANT.to_string(),
            },
            Constant {
                name: "HASH_ALGORITHM",
                value: HASH_ALGORITHM.to_string(),
            },
            Constant {
                name: "HASH_COST",
                value: HASH_COST.to_string(), // Remove this line
            },
            Constant {
                name: "HASH_LENGTH",
                value: HASH_LENGTH.to_string(),
            },
            Constant {
                name: "KHINCHIN",
                value: KHINCHIN.to_string(),
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
                name: "PLANCK_REDUCED",
                value: PLANCK_REDUCED.to_string(),
            },
            Constant {
                name: "SILVER_RATIO",
                value: SILVER_RATIO.to_string(),
            },
            Constant {
                name: "SPEED_OF_LIGHT",
                value: SPEED_OF_LIGHT.to_string(),
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
            Constant {
                name: "VACUUM_PERMEABILITY",
                value: VACUUM_PERMEABILITY.to_string(),
            },
            Constant {
                name: "VACUUM_PERMITTIVITY",
                value: VACUUM_PERMITTIVITY.to_string(),
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

/// Apéry's constant, which is the sum of the reciprocals of the positive cubes.
/// ζ(3) ≈ 1.2020569032
pub const APERY: f64 = 1.2020569031595942;

/// Avogadro's constant, which is the number of constituent particles contained in one mole of a substance.
/// N_A ≈ 6.02214076 x 10^23 mol^-1
pub const AVOGADRO: f64 = 6.02214076e23;

/// Boltzmann's constant, which relates the average kinetic energy of particles in a gas with the temperature of the gas.
/// k_B ≈ 1.380648 x 10^-23 J K^-1
pub const BOLTZMANN: f64 = 1.380648e-23;

/// Catalan's constant, which is the sum of the alternating harmonic series.
/// C ≈ 0.915965594177219
pub const CATALAN: f64 = 0.915_965_594_177_219;

/// Coulomb's constant, which is the proportionality constant in Coulomb's law.
/// k_e ≈ 8.9875517923 x 10^9 N m^2 C^-2
pub const COULOMB: f64 = 8.9875517923e9;

/// The base of the natural logarithm, Euler's number.
/// e ≈ 2.7182818284590452353602874713527
pub const EULER: f64 = std::f64::consts::E;

/// Faraday constant, which represents the amount of electric charge carried by one mole of electrons.
/// F ≈ 96485.33212 C mol^-1
pub const FARADAY: f64 = 96485.33212;

/// The Euler-Mascheroni constant, which is the limiting difference between the harmonic series and the natural logarithm.
/// γ ≈ 0.5772156649015329
pub const GAMMA: f64 = 0.5772156649015329;

/// The gas constant, which relates the energy scale to the temperature scale in the ideal gas law.
/// R ≈ 8.314462618 J mol^-1 K^-1
pub const GAS_CONSTANT: f64 = 8.314462618;

/// Glaisher-Kinkelin constant, which arises in the asymptotic expansion of the Barnes G-function.
/// A ≈ 1.2824271291
pub const GLAISHER_KINKELIN: f64 = 1.2824271291006226;

/// The gravitational constant, which is the proportionality constant in Newton's law of universal gravitation.
/// G ≈ 6.67430 x 10^-11 m^3 kg^-1 s^-2
pub const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;

/// The hash algorithm used. The default is Blake3.
pub const HASH_ALGORITHM: &str = "Blake3";

/// The cost of the hash algorithm. The default is 8.
pub const HASH_COST: u32 = 8;

/// The hash length is the length of the hash in bytes.
/// - The default is 32.
/// - The maximum is 64.
/// - The minimum is 16.
pub const HASH_LENGTH: usize = 32;

/// Khinchin's constant, which appears in the theory of continued fractions.
/// K ≈ 2.6854520010
pub const KHINCHIN: f64 = 2.6854520010653064;

/// The golden ratio, which is the limit of the ratio of consecutive Fibonacci numbers.
/// φ = (1 + √5) / 2 ≈ 1.6180339887498948482045868343656
pub const PHI: f64 = (1.0 + SQRT5) / 2.0;

/// The ratio of a circle's circumference to its diameter.
/// π ≈ 3.14159265358979323846264338327950288
pub const PI: f64 = std::f64::consts::PI;

/// Planck's constant, which relates the energy of a photon to its frequency.
/// h ≈ 6.62607015 x 10^-34 J s
pub const PLANCK: f64 = 6.62607015e-34;

/// Planck's reduced constant, which is Planck's constant divided by 2π.
/// ħ = h / (2π) ≈ 1.054571817 x 10^-34 J s
pub const PLANCK_REDUCED: f64 = PLANCK / (2.0 * PI);

/// The silver ratio, which is one of the silver means.
/// δ_s = 1 + √2 ≈ 2.4142135623730950488016887242097
pub const SILVER_RATIO: f64 = 1.0 + SQRT2;

/// The speed of light in vacuum.
/// c ≈ 299792458 m s^-1
pub const SPEED_OF_LIGHT: f64 = 299792458.0;

/// A set of special characters.
pub const SPECIAL_CHARS: &[char] = &[
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '=',
    '[', ']', '{', '}', '|', ';', ':', '"', '<', '>', ',', '.', '?',
    '/', '~', '`',
];

/// The square root of 2.
/// √2 ≈ 1.4142135623730950488016887242097
pub const SQRT2: f64 = std::f64::consts::SQRT_2;

/// The square root of 3.
/// √3 ≈ 1.7320508075688772935274463415059
pub const SQRT3: f64 = 1.7320508075688772;

/// The square root of 5.
/// √5 ≈ 2.23606797749979
pub const SQRT5: f64 = 2.236_067_977_499_79;

/// The circle constant, which is the ratio of a circle's circumference to its radius.
/// τ = 2π ≈ 6.28318530717958647692528676655900577
pub const TAU: f64 = std::f64::consts::TAU;

/// The vacuum permeability, which relates magnetic induction to magnetic field strength.
/// μ_0 ≈ 1.25663706212 x 10^-6 N A^-2
pub const VACUUM_PERMEABILITY: f64 = 1.25663706212e-6;

/// The vacuum permittivity, which relates electric displacement to electric field strength.
/// ε_0 ≈ 8.8541878128 x 10^-12 F m^-1
pub const VACUUM_PERMITTIVITY: f64 = 8.8541878128e-12;
