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

/// Collection of 110 mathematical, physical, and cryptographic constants.
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

    /// Returns a slice of all 110 constants.
    ///
    /// # Example
    ///
    /// ```
    /// use cmn::constants::Constants;
    ///
    /// let constants = Constants::new();
    /// assert_eq!(constants.constants().len(), 110);
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
    /// assert_eq!(constants.constants().len(), 110);
    ///
    /// ```
    ///
    pub fn new() -> Self {
        let mut constants = Vec::with_capacity(110);
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
            // --- Particle masses ---
            Constant {
                name: "MUON_MASS",
                value: MUON_MASS.to_string(),
            },
            Constant {
                name: "TAU_PARTICLE_MASS",
                value: TAU_PARTICLE_MASS.to_string(),
            },
            Constant {
                name: "DEUTERON_MASS",
                value: DEUTERON_MASS.to_string(),
            },
            Constant {
                name: "TRITON_MASS",
                value: TRITON_MASS.to_string(),
            },
            Constant {
                name: "HELION_MASS",
                value: HELION_MASS.to_string(),
            },
            Constant {
                name: "ALPHA_PARTICLE_MASS",
                value: ALPHA_PARTICLE_MASS.to_string(),
            },
            // --- Mass ratios ---
            Constant {
                name: "ELECTRON_PROTON_MASS_RATIO",
                value: ELECTRON_PROTON_MASS_RATIO.to_string(),
            },
            Constant {
                name: "PROTON_ELECTRON_MASS_RATIO",
                value: PROTON_ELECTRON_MASS_RATIO.to_string(),
            },
            Constant {
                name: "MUON_ELECTRON_MASS_RATIO",
                value: MUON_ELECTRON_MASS_RATIO.to_string(),
            },
            Constant {
                name: "NEUTRON_PROTON_MASS_RATIO",
                value: NEUTRON_PROTON_MASS_RATIO.to_string(),
            },
            Constant {
                name: "DEUTERON_PROTON_MASS_RATIO",
                value: DEUTERON_PROTON_MASS_RATIO.to_string(),
            },
            // --- Magnetic moments ---
            Constant {
                name: "BOHR_MAGNETON",
                value: BOHR_MAGNETON.to_string(),
            },
            Constant {
                name: "NUCLEAR_MAGNETON",
                value: NUCLEAR_MAGNETON.to_string(),
            },
            Constant {
                name: "ELECTRON_MAGNETIC_MOMENT",
                value: ELECTRON_MAGNETIC_MOMENT.to_string(),
            },
            Constant {
                name: "PROTON_MAGNETIC_MOMENT",
                value: PROTON_MAGNETIC_MOMENT.to_string(),
            },
            Constant {
                name: "NEUTRON_MAGNETIC_MOMENT",
                value: NEUTRON_MAGNETIC_MOMENT.to_string(),
            },
            Constant {
                name: "ELECTRON_G_FACTOR",
                value: ELECTRON_G_FACTOR.to_string(),
            },
            Constant {
                name: "PROTON_G_FACTOR",
                value: PROTON_G_FACTOR.to_string(),
            },
            // --- eV equivalents ---
            Constant {
                name: "ELECTRON_VOLT",
                value: ELECTRON_VOLT.to_string(),
            },
            Constant {
                name: "EV_TO_KG",
                value: EV_TO_KG.to_string(),
            },
            Constant {
                name: "EV_TO_AMU",
                value: EV_TO_AMU.to_string(),
            },
            Constant {
                name: "EV_TO_HZ",
                value: EV_TO_HZ.to_string(),
            },
            Constant {
                name: "EV_TO_KELVIN",
                value: EV_TO_KELVIN.to_string(),
            },
            Constant {
                name: "EV_TO_INVERSE_METER",
                value: EV_TO_INVERSE_METER.to_string(),
            },
            // --- Atomic & nuclear ---
            Constant {
                name: "CLASSICAL_ELECTRON_RADIUS",
                value: CLASSICAL_ELECTRON_RADIUS.to_string(),
            },
            Constant {
                name: "ELECTRON_COMPTON_WAVELENGTH",
                value: ELECTRON_COMPTON_WAVELENGTH.to_string(),
            },
            Constant {
                name: "PROTON_COMPTON_WAVELENGTH",
                value: PROTON_COMPTON_WAVELENGTH.to_string(),
            },
            Constant {
                name: "NEUTRON_COMPTON_WAVELENGTH",
                value: NEUTRON_COMPTON_WAVELENGTH.to_string(),
            },
            Constant {
                name: "THOMSON_CROSS_SECTION",
                value: THOMSON_CROSS_SECTION.to_string(),
            },
            Constant {
                name: "FIRST_RADIATION_CONSTANT",
                value: FIRST_RADIATION_CONSTANT.to_string(),
            },
            Constant {
                name: "SECOND_RADIATION_CONSTANT",
                value: SECOND_RADIATION_CONSTANT.to_string(),
            },
            Constant {
                name: "JOSEPHSON_CONSTANT",
                value: JOSEPHSON_CONSTANT.to_string(),
            },
            Constant {
                name: "VON_KLITZING_CONSTANT",
                value: VON_KLITZING_CONSTANT.to_string(),
            },
            Constant {
                name: "HARTREE_ENERGY",
                value: HARTREE_ENERGY.to_string(),
            },
            Constant {
                name: "HARTREE_ENERGY_EV",
                value: HARTREE_ENERGY_EV.to_string(),
            },
            // --- Planck units ---
            Constant {
                name: "PLANCK_MASS",
                value: PLANCK_MASS.to_string(),
            },
            Constant {
                name: "PLANCK_LENGTH",
                value: PLANCK_LENGTH.to_string(),
            },
            Constant {
                name: "PLANCK_TIME",
                value: PLANCK_TIME.to_string(),
            },
            Constant {
                name: "PLANCK_TEMPERATURE",
                value: PLANCK_TEMPERATURE.to_string(),
            },
            Constant {
                name: "PLANCK_CHARGE",
                value: PLANCK_CHARGE.to_string(),
            },
            // --- Molar & thermodynamic ---
            Constant {
                name: "MOLAR_MASS_CONSTANT",
                value: MOLAR_MASS_CONSTANT.to_string(),
            },
            Constant {
                name: "MOLAR_PLANCK_CONSTANT",
                value: MOLAR_PLANCK_CONSTANT.to_string(),
            },
            Constant {
                name: "LOSCHMIDT_CONSTANT",
                value: LOSCHMIDT_CONSTANT.to_string(),
            },
            Constant {
                name: "MOLAR_VOLUME_IDEAL_GAS",
                value: MOLAR_VOLUME_IDEAL_GAS.to_string(),
            },
            Constant {
                name: "SACKUR_TETRODE_CONSTANT",
                value: SACKUR_TETRODE_CONSTANT.to_string(),
            },
            // --- Electromagnetic additional ---
            Constant {
                name: "IMPEDANCE_OF_FREE_SPACE",
                value: IMPEDANCE_OF_FREE_SPACE.to_string(),
            },
            Constant {
                name: "INVERSE_FINE_STRUCTURE",
                value: INVERSE_FINE_STRUCTURE.to_string(),
            },
            Constant {
                name: "ELECTRON_CHARGE_TO_MASS",
                value: ELECTRON_CHARGE_TO_MASS.to_string(),
            },
            Constant {
                name: "PROTON_CHARGE_TO_MASS",
                value: PROTON_CHARGE_TO_MASS.to_string(),
            },
            // --- Atomic units ---
            Constant {
                name: "ATOMIC_UNIT_OF_LENGTH",
                value: ATOMIC_UNIT_OF_LENGTH.to_string(),
            },
            Constant {
                name: "ATOMIC_UNIT_OF_TIME",
                value: ATOMIC_UNIT_OF_TIME.to_string(),
            },
            Constant {
                name: "ATOMIC_UNIT_OF_VELOCITY",
                value: ATOMIC_UNIT_OF_VELOCITY.to_string(),
            },
            Constant {
                name: "ATOMIC_UNIT_OF_FORCE",
                value: ATOMIC_UNIT_OF_FORCE.to_string(),
            },
            Constant {
                name: "ATOMIC_UNIT_OF_ELECTRIC_FIELD",
                value: ATOMIC_UNIT_OF_ELECTRIC_FIELD.to_string(),
            },
            Constant {
                name: "ATOMIC_UNIT_OF_POLARIZABILITY",
                value: ATOMIC_UNIT_OF_POLARIZABILITY.to_string(),
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

#[cfg(feature = "std")]
impl core::fmt::Display for ConstantValue {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Float(v) => write!(f, "{v}"),
            Self::String(v) => write!(f, "{v}"),
            Self::U32(v) => write!(f, "{v}"),
            Self::Usize(v) => write!(f, "{v}"),
            Self::CharArray(v) => {
                for ch in *v {
                    write!(f, "{ch}")?;
                }
                Ok(())
            }
        }
    }
}

/// Constant category for [`CONSTANTS_TABLE`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    /// Pure mathematical constant.
    Mathematical,
    /// Physical / SI constant (CODATA 2018).
    Physical,
    /// Cryptographic or utility constant.
    Cryptographic,
}

/// A static lookup table of all named float constants with
/// category metadata.
///
/// Available in `no_std`. Zero allocation. Use this for
/// compile-time constant lookup by name without the `std`
/// feature.
///
/// # Example
///
/// ```
/// use cmn::constants::{CONSTANTS_TABLE, Category};
///
/// let pi = CONSTANTS_TABLE.iter()
///     .find(|(name, _, _)| *name == "PI")
///     .map(|(_, val, _)| *val);
/// assert_eq!(pi, Some(core::f64::consts::PI));
///
/// // Filter by category
/// let physical: Vec<_> = CONSTANTS_TABLE.iter()
///     .filter(|(_, _, cat)| *cat == Category::Physical)
///     .collect();
/// assert!(physical.len() > 10);
/// ```
use Category::{Mathematical as M, Physical as P};
/// A static lookup table of all named float constants.
pub const CONSTANTS_TABLE: &[(&str, f64, Category)] = &[
    ("APERY", APERY, M),
    ("AVOGADRO", AVOGADRO, P),
    ("BOLTZMANN", BOLTZMANN, P),
    ("CATALAN", CATALAN, M),
    ("COULOMB", COULOMB, P),
    ("EULER", EULER, M),
    ("FARADAY", FARADAY, P),
    ("GAMMA", GAMMA, M),
    ("GAS_CONSTANT", GAS_CONSTANT, P),
    ("GLAISHER_KINKELIN", GLAISHER_KINKELIN, M),
    ("GRAVITATIONAL_CONSTANT", GRAVITATIONAL_CONSTANT, P),
    ("KHINCHIN", KHINCHIN, M),
    ("PHI", PHI, M),
    ("PI", PI, M),
    ("PLANCK", PLANCK, P),
    ("PLANCK_REDUCED", PLANCK_REDUCED, P),
    ("SILVER_RATIO", SILVER_RATIO, M),
    ("SPEED_OF_LIGHT", SPEED_OF_LIGHT, P),
    ("SQRT2", SQRT2, M),
    ("SQRT3", SQRT3, M),
    ("SQRT5", SQRT5, M),
    ("TAU", TAU, M),
    ("VACUUM_PERMEABILITY", VACUUM_PERMEABILITY, P),
    ("VACUUM_PERMITTIVITY", VACUUM_PERMITTIVITY, P),
    ("LN_2", LN_2, M),
    ("LN_10", LN_10, M),
    ("LOG2_E", LOG2_E, M),
    ("LOG10_E", LOG10_E, M),
    ("FRAC_1_SQRT_2", FRAC_1_SQRT_2, M),
    ("FRAC_1_PI", FRAC_1_PI, M),
    ("FRAC_2_PI", FRAC_2_PI, M),
    ("FRAC_2_SQRT_PI", FRAC_2_SQRT_PI, M),
    ("FRAC_PI_2", FRAC_PI_2, M),
    ("FRAC_PI_3", FRAC_PI_3, M),
    ("FRAC_PI_4", FRAC_PI_4, M),
    ("FRAC_PI_6", FRAC_PI_6, M),
    ("FRAC_PI_8", FRAC_PI_8, M),
    ("ELEMENTARY_CHARGE", ELEMENTARY_CHARGE, P),
    ("ELECTRON_MASS", ELECTRON_MASS, P),
    ("PROTON_MASS", PROTON_MASS, P),
    ("NEUTRON_MASS", NEUTRON_MASS, P),
    ("STEFAN_BOLTZMANN", STEFAN_BOLTZMANN, P),
    ("WIEN_DISPLACEMENT", WIEN_DISPLACEMENT, P),
    ("STANDARD_GRAVITY", STANDARD_GRAVITY, P),
    ("STANDARD_ATMOSPHERE", STANDARD_ATMOSPHERE, P),
    ("ATOMIC_MASS_UNIT", ATOMIC_MASS_UNIT, P),
    ("BOHR_RADIUS", BOHR_RADIUS, P),
    ("FINE_STRUCTURE", FINE_STRUCTURE, P),
    ("RYDBERG", RYDBERG, P),
    ("MAGNETIC_FLUX_QUANTUM", MAGNETIC_FLUX_QUANTUM, P),
    ("CONDUCTANCE_QUANTUM", CONDUCTANCE_QUANTUM, P),
    ("MUON_MASS", MUON_MASS, P),
    ("TAU_PARTICLE_MASS", TAU_PARTICLE_MASS, P),
    ("DEUTERON_MASS", DEUTERON_MASS, P),
    ("TRITON_MASS", TRITON_MASS, P),
    ("HELION_MASS", HELION_MASS, P),
    ("ALPHA_PARTICLE_MASS", ALPHA_PARTICLE_MASS, P),
    ("ELECTRON_PROTON_MASS_RATIO", ELECTRON_PROTON_MASS_RATIO, P),
    ("PROTON_ELECTRON_MASS_RATIO", PROTON_ELECTRON_MASS_RATIO, P),
    ("MUON_ELECTRON_MASS_RATIO", MUON_ELECTRON_MASS_RATIO, P),
    ("NEUTRON_PROTON_MASS_RATIO", NEUTRON_PROTON_MASS_RATIO, P),
    ("DEUTERON_PROTON_MASS_RATIO", DEUTERON_PROTON_MASS_RATIO, P),
    ("BOHR_MAGNETON", BOHR_MAGNETON, P),
    ("NUCLEAR_MAGNETON", NUCLEAR_MAGNETON, P),
    ("ELECTRON_MAGNETIC_MOMENT", ELECTRON_MAGNETIC_MOMENT, P),
    ("PROTON_MAGNETIC_MOMENT", PROTON_MAGNETIC_MOMENT, P),
    ("NEUTRON_MAGNETIC_MOMENT", NEUTRON_MAGNETIC_MOMENT, P),
    ("ELECTRON_G_FACTOR", ELECTRON_G_FACTOR, P),
    ("PROTON_G_FACTOR", PROTON_G_FACTOR, P),
    ("ELECTRON_VOLT", ELECTRON_VOLT, P),
    ("EV_TO_KG", EV_TO_KG, P),
    ("EV_TO_AMU", EV_TO_AMU, P),
    ("EV_TO_HZ", EV_TO_HZ, P),
    ("EV_TO_KELVIN", EV_TO_KELVIN, P),
    ("EV_TO_INVERSE_METER", EV_TO_INVERSE_METER, P),
    ("CLASSICAL_ELECTRON_RADIUS", CLASSICAL_ELECTRON_RADIUS, P),
    (
        "ELECTRON_COMPTON_WAVELENGTH",
        ELECTRON_COMPTON_WAVELENGTH,
        P,
    ),
    ("PROTON_COMPTON_WAVELENGTH", PROTON_COMPTON_WAVELENGTH, P),
    ("NEUTRON_COMPTON_WAVELENGTH", NEUTRON_COMPTON_WAVELENGTH, P),
    ("THOMSON_CROSS_SECTION", THOMSON_CROSS_SECTION, P),
    ("FIRST_RADIATION_CONSTANT", FIRST_RADIATION_CONSTANT, P),
    ("SECOND_RADIATION_CONSTANT", SECOND_RADIATION_CONSTANT, P),
    ("JOSEPHSON_CONSTANT", JOSEPHSON_CONSTANT, P),
    ("VON_KLITZING_CONSTANT", VON_KLITZING_CONSTANT, P),
    ("HARTREE_ENERGY", HARTREE_ENERGY, P),
    ("HARTREE_ENERGY_EV", HARTREE_ENERGY_EV, P),
    ("PLANCK_MASS", PLANCK_MASS, P),
    ("PLANCK_LENGTH", PLANCK_LENGTH, P),
    ("PLANCK_TIME", PLANCK_TIME, P),
    ("PLANCK_TEMPERATURE", PLANCK_TEMPERATURE, P),
    ("PLANCK_CHARGE", PLANCK_CHARGE, P),
    ("MOLAR_MASS_CONSTANT", MOLAR_MASS_CONSTANT, P),
    ("MOLAR_PLANCK_CONSTANT", MOLAR_PLANCK_CONSTANT, P),
    ("LOSCHMIDT_CONSTANT", LOSCHMIDT_CONSTANT, P),
    ("MOLAR_VOLUME_IDEAL_GAS", MOLAR_VOLUME_IDEAL_GAS, P),
    ("SACKUR_TETRODE_CONSTANT", SACKUR_TETRODE_CONSTANT, P),
    ("IMPEDANCE_OF_FREE_SPACE", IMPEDANCE_OF_FREE_SPACE, P),
    ("INVERSE_FINE_STRUCTURE", INVERSE_FINE_STRUCTURE, P),
    ("ELECTRON_CHARGE_TO_MASS", ELECTRON_CHARGE_TO_MASS, P),
    ("PROTON_CHARGE_TO_MASS", PROTON_CHARGE_TO_MASS, P),
    ("ATOMIC_UNIT_OF_LENGTH", ATOMIC_UNIT_OF_LENGTH, P),
    ("ATOMIC_UNIT_OF_TIME", ATOMIC_UNIT_OF_TIME, P),
    ("ATOMIC_UNIT_OF_VELOCITY", ATOMIC_UNIT_OF_VELOCITY, P),
    ("ATOMIC_UNIT_OF_FORCE", ATOMIC_UNIT_OF_FORCE, P),
    (
        "ATOMIC_UNIT_OF_ELECTRIC_FIELD",
        ATOMIC_UNIT_OF_ELECTRIC_FIELD,
        P,
    ),
    (
        "ATOMIC_UNIT_OF_POLARIZABILITY",
        ATOMIC_UNIT_OF_POLARIZABILITY,
        P,
    ),
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

// ---------------------------------------------------------------
// Particle masses — CODATA 2018
// ---------------------------------------------------------------

/// Muon mass (CODATA 2018).
/// m_μ ≈ 1.883531627e-28 kg
pub const MUON_MASS: f64 = 1.883_531_627e-28;

/// Tau lepton mass (CODATA 2018).
/// m_τ ≈ 3.16754e-27 kg
pub const TAU_PARTICLE_MASS: f64 = 3.167_54e-27;

/// Deuteron mass (CODATA 2018).
/// m_d ≈ 3.3435837724e-27 kg
pub const DEUTERON_MASS: f64 = 3.343_583_772_4e-27;

/// Triton mass (CODATA 2018).
/// m_t ≈ 5.0073567446e-27 kg
pub const TRITON_MASS: f64 = 5.007_356_744_6e-27;

/// Helion (He-3 nucleus) mass (CODATA 2018).
/// m_h ≈ 5.0064127796e-27 kg
pub const HELION_MASS: f64 = 5.006_412_779_6e-27;

/// Alpha particle mass (CODATA 2018).
/// m_α ≈ 6.6446573357e-27 kg
pub const ALPHA_PARTICLE_MASS: f64 = 6.644_657_335_7e-27;

// ---------------------------------------------------------------
// Mass ratios — CODATA 2018
// ---------------------------------------------------------------

/// Electron-to-proton mass ratio (CODATA 2018).
/// m_e / m_p ≈ 5.44617021487e-4
pub const ELECTRON_PROTON_MASS_RATIO: f64 = 5.446_170_214_87e-4;

/// Proton-to-electron mass ratio (CODATA 2018).
/// m_p / m_e ≈ 1836.15267343
pub const PROTON_ELECTRON_MASS_RATIO: f64 = 1836.152_673_43;

/// Muon-to-electron mass ratio (CODATA 2018).
/// m_μ / m_e ≈ 206.7682830
pub const MUON_ELECTRON_MASS_RATIO: f64 = 206.768_283_0;

/// Neutron-to-proton mass ratio (CODATA 2018).
/// m_n / m_p ≈ 1.00137841931
pub const NEUTRON_PROTON_MASS_RATIO: f64 = 1.001_378_419_31;

/// Deuteron-to-proton mass ratio (CODATA 2018).
/// m_d / m_p ≈ 1.99900750139
pub const DEUTERON_PROTON_MASS_RATIO: f64 = 1.999_007_501_39;

// ---------------------------------------------------------------
// Magnetic moments — CODATA 2018
// ---------------------------------------------------------------

/// Bohr magneton (CODATA 2018).
/// μ_B = eℏ/(2m_e) ≈ 9.2740100783e-24 J/T
pub const BOHR_MAGNETON: f64 = 9.274_010_078_3e-24;

/// Nuclear magneton (CODATA 2018).
/// μ_N = eℏ/(2m_p) ≈ 5.0507837461e-27 J/T
pub const NUCLEAR_MAGNETON: f64 = 5.050_783_746_1e-27;

/// Electron magnetic moment (CODATA 2018).
/// μ_e ≈ -9.2847647043e-24 J/T
pub const ELECTRON_MAGNETIC_MOMENT: f64 = -9.284_764_704_3e-24;

/// Proton magnetic moment (CODATA 2018).
/// μ_p ≈ 1.41060679736e-26 J/T
pub const PROTON_MAGNETIC_MOMENT: f64 = 1.410_606_797_36e-26;

/// Neutron magnetic moment (CODATA 2018).
/// μ_n ≈ -9.6623651e-27 J/T
pub const NEUTRON_MAGNETIC_MOMENT: f64 = -9.662_365_1e-27;

/// Electron g-factor (CODATA 2018).
/// g_e ≈ -2.00231930436256
pub const ELECTRON_G_FACTOR: f64 = -2.002_319_304_362_56;

/// Proton g-factor (CODATA 2018).
/// g_p ≈ 5.5856946893
pub const PROTON_G_FACTOR: f64 = 5.585_694_689_3;

// ---------------------------------------------------------------
// Electron volt equivalents — CODATA 2018 (exact since 2019)
// ---------------------------------------------------------------

/// Electron volt in joules (CODATA 2018, exact).
/// 1 eV = 1.602176634e-19 J
pub const ELECTRON_VOLT: f64 = 1.602_176_634e-19;

/// Electron volt–kilogram relationship (exact).
/// 1 eV/c² ≈ 1.782661921e-36 kg
pub const EV_TO_KG: f64 = 1.782_661_921e-36;

/// Electron volt–atomic mass unit relationship.
/// 1 eV/c² ≈ 1.07354410233e-9 u
pub const EV_TO_AMU: f64 = 1.073_544_102_33e-9;

/// Electron volt–hertz relationship (exact).
/// 1 eV/h ≈ 2.417989242e14 Hz
pub const EV_TO_HZ: f64 = 2.417_989_242e14;

/// Electron volt–kelvin relationship (exact).
/// 1 eV/k_B ≈ 1.160451812e4 K
pub const EV_TO_KELVIN: f64 = 1.160_451_812e4;

/// Electron volt–inverse meter relationship (exact).
/// 1 eV/(hc) ≈ 8.065543937e5 /m
pub const EV_TO_INVERSE_METER: f64 = 8.065_543_937e5;

// ---------------------------------------------------------------
// Atomic & nuclear constants — CODATA 2018
// ---------------------------------------------------------------

/// Classical electron radius (CODATA 2018).
/// r_e = α²a_0 ≈ 2.8179403262e-15 m
pub const CLASSICAL_ELECTRON_RADIUS: f64 = 2.817_940_326_2e-15;

/// Compton wavelength of the electron (CODATA 2018).
/// λ_C = h/(m_e c) ≈ 2.42631023867e-12 m
pub const ELECTRON_COMPTON_WAVELENGTH: f64 = 2.426_310_238_67e-12;

/// Compton wavelength of the proton (CODATA 2018).
/// λ_C,p = h/(m_p c) ≈ 1.32140985539e-15 m
pub const PROTON_COMPTON_WAVELENGTH: f64 = 1.321_409_855_39e-15;

/// Compton wavelength of the neutron (CODATA 2018).
/// λ_C,n = h/(m_n c) ≈ 1.31959090581e-15 m
pub const NEUTRON_COMPTON_WAVELENGTH: f64 = 1.319_590_905_81e-15;

/// Thomson cross section (CODATA 2018).
/// σ_T = (8π/3)r_e² ≈ 6.6524587321e-29 m²
pub const THOMSON_CROSS_SECTION: f64 = 6.652_458_732_1e-29;

/// First radiation constant (CODATA 2018).
/// c_1 = 2πhc² ≈ 3.741771852e-16 W m²
pub const FIRST_RADIATION_CONSTANT: f64 = 3.741_771_852e-16;

/// Second radiation constant (CODATA 2018).
/// c_2 = hc/k_B ≈ 1.438776877e-2 m K
pub const SECOND_RADIATION_CONSTANT: f64 = 1.438_776_877e-2;

/// Josephson constant (CODATA 2018, exact since 2019 SI).
/// K_J = 2e/h ≈ 4.835978484e14 Hz/V
pub const JOSEPHSON_CONSTANT: f64 = 4.835_978_484e14;

/// Von Klitzing constant (CODATA 2018, exact since 2019 SI).
/// R_K = h/e² ≈ 25812.80745 Ω
pub const VON_KLITZING_CONSTANT: f64 = 25_812.807_45;

/// Hartree energy (CODATA 2018).
/// E_h = m_e c² α² ≈ 4.3597447222071e-18 J
pub const HARTREE_ENERGY: f64 = 4.359_744_722_207_1e-18;

/// Hartree energy in eV (CODATA 2018).
/// E_h ≈ 27.211386245988 eV
pub const HARTREE_ENERGY_EV: f64 = 27.211_386_245_988;

// ---------------------------------------------------------------
// Planck units — derived from fundamental constants
// ---------------------------------------------------------------

/// Planck mass.
/// m_P = √(ℏc/G) ≈ 2.176434e-8 kg
pub const PLANCK_MASS: f64 = 2.176_434e-8;

/// Planck length.
/// l_P = √(ℏG/c³) ≈ 1.616255e-35 m
pub const PLANCK_LENGTH: f64 = 1.616_255e-35;

/// Planck time.
/// t_P = √(ℏG/c⁵) ≈ 5.391247e-44 s
pub const PLANCK_TIME: f64 = 5.391_247e-44;

/// Planck temperature.
/// T_P = m_P c²/k_B ≈ 1.416784e32 K
pub const PLANCK_TEMPERATURE: f64 = 1.416_784e32;

/// Planck charge.
/// q_P = √(4πε₀ℏc) ≈ 1.875546e-18 C
pub const PLANCK_CHARGE: f64 = 1.875_546e-18;

// ---------------------------------------------------------------
// Molar & thermodynamic — CODATA 2018
// ---------------------------------------------------------------

/// Molar mass constant (CODATA 2018).
/// M_u ≈ 0.99999999965e-3 kg/mol
pub const MOLAR_MASS_CONSTANT: f64 = 0.999_999_999_65e-3;

/// Molar Planck constant (exact).
/// N_A h ≈ 3.990312712e-10 J s/mol
pub const MOLAR_PLANCK_CONSTANT: f64 = 3.990_312_712e-10;

/// Loschmidt constant at 273.15 K, 101.325 kPa.
/// n_0 ≈ 2.6867774e25 /m³
pub const LOSCHMIDT_CONSTANT: f64 = 2.686_777_4e25;

/// Molar volume of ideal gas at STP (273.15 K, 100 kPa).
/// V_m ≈ 22.71095e-3 m³/mol
pub const MOLAR_VOLUME_IDEAL_GAS: f64 = 22.710_95e-3;

/// Sackur-Tetrode constant at 1 K, 101.325 kPa.
/// S_0/R ≈ -1.15170753706
pub const SACKUR_TETRODE_CONSTANT: f64 = -1.151_707_537_06;

// ---------------------------------------------------------------
// Electromagnetic — additional CODATA 2018
// ---------------------------------------------------------------

/// Impedance of free space (CODATA 2018).
/// Z_0 = μ₀c ≈ 376.730313668 Ω
pub const IMPEDANCE_OF_FREE_SPACE: f64 = 376.730_313_668;

/// Inverse fine-structure constant (CODATA 2018).
/// 1/α ≈ 137.035999084
pub const INVERSE_FINE_STRUCTURE: f64 = 137.035_999_084;

/// Electron charge-to-mass quotient (CODATA 2018).
/// e/m_e ≈ -1.75882001076e11 C/kg
pub const ELECTRON_CHARGE_TO_MASS: f64 = -1.758_820_010_76e11;

/// Proton charge-to-mass quotient (CODATA 2018).
/// e/m_p ≈ 9.5788332e7 C/kg
pub const PROTON_CHARGE_TO_MASS: f64 = 9.578_833_2e7;

// ---------------------------------------------------------------
// Atomic unit conversions — CODATA 2018
// ---------------------------------------------------------------

/// Atomic unit of length (= Bohr radius).
/// a_0 ≈ 5.29177210903e-11 m
pub const ATOMIC_UNIT_OF_LENGTH: f64 = BOHR_RADIUS;

/// Atomic unit of time.
/// ℏ/E_h ≈ 2.4188843265857e-17 s
pub const ATOMIC_UNIT_OF_TIME: f64 = 2.418_884_326_585_7e-17;

/// Atomic unit of velocity.
/// a_0 E_h/ℏ ≈ 2.18769126364e6 m/s
pub const ATOMIC_UNIT_OF_VELOCITY: f64 = 2.187_691_263_64e6;

/// Atomic unit of force.
/// E_h/a_0 ≈ 8.2387234983e-8 N
pub const ATOMIC_UNIT_OF_FORCE: f64 = 8.238_723_498_3e-8;

/// Atomic unit of electric field.
/// E_h/(ea_0) ≈ 5.14220674763e11 V/m
pub const ATOMIC_UNIT_OF_ELECTRIC_FIELD: f64 = 5.142_206_747_63e11;

/// Atomic unit of electric polarizability.
/// e²a₀²/E_h ≈ 1.64877727436e-41 C²m²/J
pub const ATOMIC_UNIT_OF_POLARIZABILITY: f64 = 1.648_777_274_36e-41;
