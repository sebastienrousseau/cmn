// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// A single named constant with its string-encoded value.
/// Requires the `std` feature.
#[cfg(feature = "std")]
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

/// Collection of 55 mathematical, physical, and cryptographic constants.
/// Requires the `std` feature.
#[cfg(feature = "std")]
#[derive(Clone, Serialize, Debug)]
pub struct Constants {
    /// A vector of constants.
    pub constants: Vec<Constant>,
}

#[cfg(feature = "std")]
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

    /// Looks up a single constant by name.
    ///
    /// Returns `Some(Constant)` if found, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use cmn::constants::Constants;
    ///
    /// let constants = Constants::new();
    /// let euler = constants.constant("EULER");
    /// assert_eq!(euler.unwrap().name, "EULER");
    ///
    /// assert!(constants.constant("NONEXISTENT").is_none());
    /// ```
    pub fn constant(&self, name: &str) -> Option<Constant> {
        self.constants
            .iter()
            .find(|constant| constant.name == name)
            .cloned()
    }

    /// Returns a slice of all 55 constants.
    ///
    /// # Example
    ///
    /// ```
    /// use cmn::constants::Constants;
    ///
    /// let constants = Constants::new();
    /// assert_eq!(constants.constants().len(), 55);
    /// ```
    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }

    /// Create a new instance of the `Constants` structure.
    ///
    /// # Example
    ///
    /// ```
    /// use cmn::constants::Constants;
    ///
    /// let constants = Constants::new();
    /// assert_eq!(constants.constants().len(), 55);
    ///
    /// ```
    ///
    pub fn new() -> Self {
        let mut constants = Vec::with_capacity(55);
        constants.extend([
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
                value: HASH_COST.to_string(),
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
            // --- std::f64::consts math constants ---
            Constant {
                name: "LN_2",
                value: LN_2.to_string(),
            },
            Constant {
                name: "LN_10",
                value: LN_10.to_string(),
            },
            Constant {
                name: "LOG2_E",
                value: LOG2_E.to_string(),
            },
            Constant {
                name: "LOG10_E",
                value: LOG10_E.to_string(),
            },
            Constant {
                name: "FRAC_1_SQRT_2",
                value: FRAC_1_SQRT_2.to_string(),
            },
            Constant {
                name: "FRAC_1_PI",
                value: FRAC_1_PI.to_string(),
            },
            Constant {
                name: "FRAC_2_PI",
                value: FRAC_2_PI.to_string(),
            },
            Constant {
                name: "FRAC_2_SQRT_PI",
                value: FRAC_2_SQRT_PI.to_string(),
            },
            Constant {
                name: "FRAC_PI_2",
                value: FRAC_PI_2.to_string(),
            },
            Constant {
                name: "FRAC_PI_3",
                value: FRAC_PI_3.to_string(),
            },
            Constant {
                name: "FRAC_PI_4",
                value: FRAC_PI_4.to_string(),
            },
            Constant {
                name: "FRAC_PI_6",
                value: FRAC_PI_6.to_string(),
            },
            Constant {
                name: "FRAC_PI_8",
                value: FRAC_PI_8.to_string(),
            },
            // --- CODATA 2018 physical constants ---
            Constant {
                name: "ELEMENTARY_CHARGE",
                value: ELEMENTARY_CHARGE.to_string(),
            },
            Constant {
                name: "ELECTRON_MASS",
                value: ELECTRON_MASS.to_string(),
            },
            Constant {
                name: "PROTON_MASS",
                value: PROTON_MASS.to_string(),
            },
            Constant {
                name: "NEUTRON_MASS",
                value: NEUTRON_MASS.to_string(),
            },
            Constant {
                name: "STEFAN_BOLTZMANN",
                value: STEFAN_BOLTZMANN.to_string(),
            },
            Constant {
                name: "WIEN_DISPLACEMENT",
                value: WIEN_DISPLACEMENT.to_string(),
            },
            Constant {
                name: "STANDARD_GRAVITY",
                value: STANDARD_GRAVITY.to_string(),
            },
            Constant {
                name: "STANDARD_ATMOSPHERE",
                value: STANDARD_ATMOSPHERE.to_string(),
            },
            Constant {
                name: "ATOMIC_MASS_UNIT",
                value: ATOMIC_MASS_UNIT.to_string(),
            },
            Constant {
                name: "BOHR_RADIUS",
                value: BOHR_RADIUS.to_string(),
            },
            Constant {
                name: "FINE_STRUCTURE",
                value: FINE_STRUCTURE.to_string(),
            },
            Constant {
                name: "RYDBERG",
                value: RYDBERG.to_string(),
            },
            Constant {
                name: "MAGNETIC_FLUX_QUANTUM",
                value: MAGNETIC_FLUX_QUANTUM.to_string(),
            },
            Constant {
                name: "CONDUCTANCE_QUANTUM",
                value: CONDUCTANCE_QUANTUM.to_string(),
            },
        ]);

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

#[cfg(feature = "std")]
impl Default for Constants {
    fn default() -> Self {
        Self::new()
    }
}

/// Enum to represent the different constant values.
/// Requires the `std` feature.
#[cfg(feature = "std")]
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

/// A static lookup table of all named float constants.
///
/// Available in `no_std`. Zero allocation. Use this for
/// compile-time constant lookup by name without the `std`
/// feature.
///
/// # Example
///
/// ```
/// use cmn::constants::CONSTANTS_TABLE;
///
/// let pi = CONSTANTS_TABLE.iter()
///     .find(|(name, _)| *name == "PI")
///     .map(|(_, val)| *val);
/// assert_eq!(pi, Some(core::f64::consts::PI));
/// ```
pub const CONSTANTS_TABLE: &[(&str, f64)] = &[
    ("APERY", APERY),
    ("AVOGADRO", AVOGADRO),
    ("BOLTZMANN", BOLTZMANN),
    ("CATALAN", CATALAN),
    ("COULOMB", COULOMB),
    ("EULER", EULER),
    ("FARADAY", FARADAY),
    ("GAMMA", GAMMA),
    ("GAS_CONSTANT", GAS_CONSTANT),
    ("GLAISHER_KINKELIN", GLAISHER_KINKELIN),
    ("GRAVITATIONAL_CONSTANT", GRAVITATIONAL_CONSTANT),
    ("KHINCHIN", KHINCHIN),
    ("PHI", PHI),
    ("PI", PI),
    ("PLANCK", PLANCK),
    ("PLANCK_REDUCED", PLANCK_REDUCED),
    ("SILVER_RATIO", SILVER_RATIO),
    ("SPEED_OF_LIGHT", SPEED_OF_LIGHT),
    ("SQRT2", SQRT2),
    ("SQRT3", SQRT3),
    ("SQRT5", SQRT5),
    ("TAU", TAU),
    ("VACUUM_PERMEABILITY", VACUUM_PERMEABILITY),
    ("VACUUM_PERMITTIVITY", VACUUM_PERMITTIVITY),
    ("LN_2", LN_2),
    ("LN_10", LN_10),
    ("LOG2_E", LOG2_E),
    ("LOG10_E", LOG10_E),
    ("FRAC_1_SQRT_2", FRAC_1_SQRT_2),
    ("FRAC_1_PI", FRAC_1_PI),
    ("FRAC_2_PI", FRAC_2_PI),
    ("FRAC_2_SQRT_PI", FRAC_2_SQRT_PI),
    ("FRAC_PI_2", FRAC_PI_2),
    ("FRAC_PI_3", FRAC_PI_3),
    ("FRAC_PI_4", FRAC_PI_4),
    ("FRAC_PI_6", FRAC_PI_6),
    ("FRAC_PI_8", FRAC_PI_8),
    ("ELEMENTARY_CHARGE", ELEMENTARY_CHARGE),
    ("ELECTRON_MASS", ELECTRON_MASS),
    ("PROTON_MASS", PROTON_MASS),
    ("NEUTRON_MASS", NEUTRON_MASS),
    ("STEFAN_BOLTZMANN", STEFAN_BOLTZMANN),
    ("WIEN_DISPLACEMENT", WIEN_DISPLACEMENT),
    ("STANDARD_GRAVITY", STANDARD_GRAVITY),
    ("STANDARD_ATMOSPHERE", STANDARD_ATMOSPHERE),
    ("ATOMIC_MASS_UNIT", ATOMIC_MASS_UNIT),
    ("BOHR_RADIUS", BOHR_RADIUS),
    ("FINE_STRUCTURE", FINE_STRUCTURE),
    ("RYDBERG", RYDBERG),
    ("MAGNETIC_FLUX_QUANTUM", MAGNETIC_FLUX_QUANTUM),
    ("CONDUCTANCE_QUANTUM", CONDUCTANCE_QUANTUM),
];

/// Apéry's constant, which is the sum of the reciprocals of the positive cubes.
/// ζ(3) ≈ 1.2020569032
pub const APERY: f64 = 1.2020569031595942;

/// Avogadro's constant (CODATA 2018, exact since 2019 SI).
/// N_A = 6.02214076 x 10^23 mol^-1
pub const AVOGADRO: f64 = 6.02214076e23;

/// Boltzmann's constant (CODATA 2018, exact since 2019 SI).
/// k_B = 1.380649 x 10^-23 J K^-1
pub const BOLTZMANN: f64 = 1.380_649e-23;

/// Catalan's constant, which is the sum of the alternating harmonic series.
/// C ≈ 0.915965594177219
pub const CATALAN: f64 = 0.915_965_594_177_219;

/// Coulomb's constant (CODATA 2018).
/// k_e ≈ 8.9875517923 x 10^9 N m^2 C^-2
pub const COULOMB: f64 = 8.9875517923e9;
/// Standard uncertainty of [`COULOMB`]. Source: CODATA 2018.
pub const COULOMB_UNCERTAINTY: f64 = 0.0;

/// The base of the natural logarithm, Euler's number.
/// e ≈ 2.7182818284590452353602874713527
pub const EULER: f64 = core::f64::consts::E;

/// Faraday constant (CODATA 2018, exact since 2019 SI).
/// F = N_A * e = 96485.33212 C mol^-1
pub const FARADAY: f64 = 96485.33212;

/// The Euler-Mascheroni constant, which is the limiting difference between the harmonic series and the natural logarithm.
/// γ ≈ 0.5772156649015329
pub const GAMMA: f64 = 0.5772156649015329;

/// The molar gas constant (CODATA 2018, exact since 2019 SI).
/// R = N_A * k_B = 8.314462618 J mol^-1 K^-1
pub const GAS_CONSTANT: f64 = 8.314462618;

/// Glaisher-Kinkelin constant, which arises in the asymptotic expansion of the Barnes G-function.
/// A ≈ 1.2824271291
pub const GLAISHER_KINKELIN: f64 = 1.2824271291006226;

/// The Newtonian gravitational constant (CODATA 2018).
/// G ≈ 6.67430 x 10^-11 m^3 kg^-1 s^-2
pub const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;
/// Standard uncertainty of [`GRAVITATIONAL_CONSTANT`]. Source: CODATA 2018.
pub const GRAVITATIONAL_CONSTANT_UNCERTAINTY: f64 = 0.00015e-11;

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
pub const PI: f64 = core::f64::consts::PI;

/// Planck's constant (CODATA 2018, exact since 2019 SI).
/// h = 6.62607015 x 10^-34 J s
pub const PLANCK: f64 = 6.62607015e-34;

/// Planck's reduced constant, which is Planck's constant divided by 2π.
/// ħ = h / (2π) ≈ 1.054571817 x 10^-34 J s
pub const PLANCK_REDUCED: f64 = PLANCK / (2.0 * PI);

/// The silver ratio, which is one of the silver means.
/// δ_s = 1 + √2 ≈ 2.4142135623730950488016887242097
pub const SILVER_RATIO: f64 = 1.0 + SQRT2;

/// The speed of light in vacuum (exact since 1983 SI definition).
/// c = 299792458 m s^-1
pub const SPEED_OF_LIGHT: f64 = 299792458.0;

/// A set of special characters.
pub const SPECIAL_CHARS: &[char] = &[
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '=',
    '[', ']', '{', '}', '|', ';', ':', '"', '<', '>', ',', '.', '?',
    '/', '~', '`',
];

/// The square root of 2.
/// √2 ≈ 1.4142135623730950488016887242097
pub const SQRT2: f64 = core::f64::consts::SQRT_2;

/// The square root of 3.
/// √3 ≈ 1.7320508075688772935274463415059
pub const SQRT3: f64 = 1.7320508075688772;

/// The square root of 5.
/// √5 ≈ 2.23606797749979
pub const SQRT5: f64 = 2.236_067_977_499_79;

/// The circle constant, which is the ratio of a circle's circumference to its radius.
/// τ = 2π ≈ 6.28318530717958647692528676655900577
pub const TAU: f64 = core::f64::consts::TAU;

/// The vacuum permeability (CODATA 2018).
/// μ_0 ≈ 1.25663706212 x 10^-6 N A^-2
pub const VACUUM_PERMEABILITY: f64 = 1.25663706212e-6;
/// Standard uncertainty of [`VACUUM_PERMEABILITY`]. Source: CODATA 2018.
pub const VACUUM_PERMEABILITY_UNCERTAINTY: f64 = 0.00000000019e-6;

/// The vacuum permittivity (CODATA 2018).
/// ε_0 ≈ 8.8541878128 x 10^-12 F m^-1
pub const VACUUM_PERMITTIVITY: f64 = 8.8541878128e-12;
/// Standard uncertainty of [`VACUUM_PERMITTIVITY`]. Source: CODATA 2018.
pub const VACUUM_PERMITTIVITY_UNCERTAINTY: f64 = 0.0000000013e-12;

// ---------------------------------------------------------------
// Mathematical constants from std::f64::consts
// ---------------------------------------------------------------

/// The natural logarithm of 2.
/// ln(2) ≈ 0.6931471805599453
pub const LN_2: f64 = core::f64::consts::LN_2;

/// The natural logarithm of 10.
/// ln(10) ≈ 2.302585092994046
pub const LN_10: f64 = core::f64::consts::LN_10;

/// The base-2 logarithm of Euler's number.
/// log₂(e) ≈ 1.4426950408889634
pub const LOG2_E: f64 = core::f64::consts::LOG2_E;

/// The base-10 logarithm of Euler's number.
/// log₁₀(e) ≈ 0.4342944819032518
pub const LOG10_E: f64 = core::f64::consts::LOG10_E;

/// The reciprocal of the square root of 2.
/// 1/√2 ≈ 0.7071067811865476
pub const FRAC_1_SQRT_2: f64 = core::f64::consts::FRAC_1_SQRT_2;

/// The reciprocal of pi.
/// 1/π ≈ 0.3183098861837907
pub const FRAC_1_PI: f64 = core::f64::consts::FRAC_1_PI;

/// Two divided by pi.
/// 2/π ≈ 0.6366197723675814
pub const FRAC_2_PI: f64 = core::f64::consts::FRAC_2_PI;

/// Two divided by the square root of pi.
/// 2/√π ≈ 1.1283791670955126
pub const FRAC_2_SQRT_PI: f64 = core::f64::consts::FRAC_2_SQRT_PI;

/// Pi divided by 2 (a right angle in radians).
/// π/2 ≈ 1.5707963267948966
pub const FRAC_PI_2: f64 = core::f64::consts::FRAC_PI_2;

/// Pi divided by 3 (60 degrees in radians).
/// π/3 ≈ 1.0471975511965979
pub const FRAC_PI_3: f64 = core::f64::consts::FRAC_PI_3;

/// Pi divided by 4 (45 degrees in radians).
/// π/4 ≈ 0.7853981633974483
pub const FRAC_PI_4: f64 = core::f64::consts::FRAC_PI_4;

/// Pi divided by 6 (30 degrees in radians).
/// π/6 ≈ 0.5235987755982989
pub const FRAC_PI_6: f64 = core::f64::consts::FRAC_PI_6;

/// Pi divided by 8 (22.5 degrees in radians).
/// π/8 ≈ 0.39269908169872414
pub const FRAC_PI_8: f64 = core::f64::consts::FRAC_PI_8;

// ---------------------------------------------------------------
// Physical constants — CODATA 2018 recommended values
// ---------------------------------------------------------------

/// The elementary charge (CODATA 2018, exact since 2019 SI).
/// e = 1.602176634 x 10^-19 C
pub const ELEMENTARY_CHARGE: f64 = 1.602_176_634e-19;

/// The rest mass of an electron (CODATA 2018).
/// m_e ≈ 9.1093837015 x 10^-31 kg
pub const ELECTRON_MASS: f64 = 9.109_383_701_5e-31;
/// Standard uncertainty of [`ELECTRON_MASS`]. Source: CODATA 2018.
pub const ELECTRON_MASS_UNCERTAINTY: f64 = 0.000_000_002_8e-31;

/// The rest mass of a proton (CODATA 2018).
/// m_p ≈ 1.67262192369 x 10^-27 kg
pub const PROTON_MASS: f64 = 1.672_621_923_69e-27;
/// Standard uncertainty of [`PROTON_MASS`]. Source: CODATA 2018.
pub const PROTON_MASS_UNCERTAINTY: f64 = 0.000_000_000_51e-27;

/// The rest mass of a neutron (CODATA 2018).
/// m_n ≈ 1.67492749804 x 10^-27 kg
pub const NEUTRON_MASS: f64 = 1.674_927_498_04e-27;
/// Standard uncertainty of [`NEUTRON_MASS`]. Source: CODATA 2018.
pub const NEUTRON_MASS_UNCERTAINTY: f64 = 0.000_000_000_95e-27;

/// The Stefan-Boltzmann constant (CODATA 2018, exact since 2019 SI).
/// σ = 5.670374419 x 10^-8 W m^-2 K^-4
pub const STEFAN_BOLTZMANN: f64 = 5.670_374_419e-8;

/// Wien's displacement constant (CODATA 2018).
/// b ≈ 2.897771955 x 10^-3 m K
pub const WIEN_DISPLACEMENT: f64 = 2.897_771_955e-3;
/// Standard uncertainty of [`WIEN_DISPLACEMENT`]. Source: CODATA 2018.
pub const WIEN_DISPLACEMENT_UNCERTAINTY: f64 = 0.000_000_013e-3;

/// Standard acceleration of gravity (exact by definition).
/// g = 9.80665 m s^-2
pub const STANDARD_GRAVITY: f64 = 9.806_65;

/// Standard atmosphere pressure (exact by definition).
/// atm = 101325 Pa
pub const STANDARD_ATMOSPHERE: f64 = 101_325.0;

/// The unified atomic mass unit / dalton (CODATA 2018).
/// u ≈ 1.66053906660 x 10^-27 kg
pub const ATOMIC_MASS_UNIT: f64 = 1.660_539_066_60e-27;
/// Standard uncertainty of [`ATOMIC_MASS_UNIT`]. Source: CODATA 2018.
pub const ATOMIC_MASS_UNIT_UNCERTAINTY: f64 = 0.000_000_000_50e-27;

/// The Bohr radius (CODATA 2018).
/// a_0 ≈ 5.29177210903 x 10^-11 m
pub const BOHR_RADIUS: f64 = 5.291_772_109_03e-11;
/// Standard uncertainty of [`BOHR_RADIUS`]. Source: CODATA 2018.
pub const BOHR_RADIUS_UNCERTAINTY: f64 = 0.000_000_000_80e-11;

/// The fine-structure constant (CODATA 2018).
/// α ≈ 7.2973525693 x 10^-3 (dimensionless)
pub const FINE_STRUCTURE: f64 = 7.297_352_569_3e-3;
/// Standard uncertainty of [`FINE_STRUCTURE`]. Source: CODATA 2018.
pub const FINE_STRUCTURE_UNCERTAINTY: f64 = 0.000_000_001_1e-3;

/// The Rydberg constant (CODATA 2018).
/// R∞ ≈ 10973731.568160 m^-1
pub const RYDBERG: f64 = 10_973_731.568_160;
/// Standard uncertainty of [`RYDBERG`]. Source: CODATA 2018.
pub const RYDBERG_UNCERTAINTY: f64 = 0.000_021;

/// The magnetic flux quantum (CODATA 2018, exact since 2019 SI).
/// Φ_0 = h/(2e) = 2.067833848 x 10^-15 Wb
pub const MAGNETIC_FLUX_QUANTUM: f64 = 2.067_833_848e-15;

/// The conductance quantum (CODATA 2018, exact since 2019 SI).
/// G_0 = 2e²/h = 7.748091729 x 10^-5 S
pub const CONDUCTANCE_QUANTUM: f64 = 7.748_091_729e-5;
