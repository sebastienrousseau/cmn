#![allow(missing_docs)]
// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(test)]
mod tests {
    use cmn::constants::{
        Constant, ConstantValue, Constants, APERY, ATOMIC_MASS_UNIT,
        AVOGADRO, BOHR_RADIUS, BOLTZMANN, CATALAN, CONDUCTANCE_QUANTUM,
        COULOMB, ELECTRON_MASS, ELEMENTARY_CHARGE, EULER, FARADAY,
        FINE_STRUCTURE, FRAC_1_PI, FRAC_1_SQRT_2, FRAC_2_PI,
        FRAC_2_SQRT_PI, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6,
        FRAC_PI_8, GAMMA, GAS_CONSTANT, GLAISHER_KINKELIN,
        GRAVITATIONAL_CONSTANT, HASH_ALGORITHM, HASH_COST, HASH_LENGTH,
        KHINCHIN, LN_10, LN_2, LOG10_E, LOG2_E, MAGNETIC_FLUX_QUANTUM,
        NEUTRON_MASS, PHI, PI, PLANCK, PLANCK_REDUCED, PROTON_MASS,
        RYDBERG, SILVER_RATIO, SPECIAL_CHARS, SPEED_OF_LIGHT, SQRT2,
        SQRT3, SQRT5, STANDARD_ATMOSPHERE, STANDARD_GRAVITY,
        STEFAN_BOLTZMANN, TAU, VACUUM_PERMEABILITY,
        VACUUM_PERMITTIVITY, WIEN_DISPLACEMENT,
    };

    /// Helper: get a Constant by name from a fresh Constants instance.
    fn get(name: &str) -> Constant {
        Constants::new().constant(name).unwrap()
    }

    // ---------------------------------------------------------------
    // Constants::new / Constants::default
    // ---------------------------------------------------------------

    #[test]
    fn new_creates_55_constants() {
        let c = Constants::new();
        assert_eq!(c.constants().len(), 110);
    }

    #[test]
    fn default_delegates_to_new() {
        let a = Constants::new();
        let b = Constants::default();
        assert_eq!(a.constants(), b.constants());
    }

    // ---------------------------------------------------------------
    // Constants::constant — found and not-found
    // ---------------------------------------------------------------

    #[test]
    fn constant_existing_returns_some() {
        let c = Constants::new();
        let euler = c.constant("EULER");
        assert!(euler.is_some());
        assert_eq!(euler.unwrap().name, "EULER");
    }

    #[test]
    fn constant_nonexistent_returns_none() {
        let c = Constants::new();
        assert!(c.constant("DOES_NOT_EXIST").is_none());
    }

    #[test]
    fn constant_empty_name_returns_none() {
        let c = Constants::new();
        assert!(c.constant("").is_none());
    }

    // ---------------------------------------------------------------
    // Constants::constants — returns all
    // ---------------------------------------------------------------

    #[test]
    fn constants_contains_all_expected_names() {
        let c = Constants::new();
        let names: Vec<&str> =
            c.constants().iter().map(|c| c.name).collect();
        let expected = [
            "APERY",
            "ATOMIC_MASS_UNIT",
            "AVOGADRO",
            "BOHR_RADIUS",
            "BOLTZMANN",
            "CATALAN",
            "CONDUCTANCE_QUANTUM",
            "COULOMB",
            "ELECTRON_MASS",
            "ELEMENTARY_CHARGE",
            "EULER",
            "FARADAY",
            "FINE_STRUCTURE",
            "FRAC_1_PI",
            "FRAC_1_SQRT_2",
            "FRAC_2_PI",
            "FRAC_2_SQRT_PI",
            "FRAC_PI_2",
            "FRAC_PI_3",
            "FRAC_PI_4",
            "FRAC_PI_6",
            "FRAC_PI_8",
            "GAMMA",
            "GAS_CONSTANT",
            "GLAISHER_KINKELIN",
            "GRAVITATIONAL_CONSTANT",
            "HASH_ALGORITHM",
            "HASH_COST",
            "HASH_LENGTH",
            "KHINCHIN",
            "LN_2",
            "LN_10",
            "LOG2_E",
            "LOG10_E",
            "MAGNETIC_FLUX_QUANTUM",
            "NEUTRON_MASS",
            "PHI",
            "PI",
            "PLANCK",
            "PLANCK_REDUCED",
            "PROTON_MASS",
            "RYDBERG",
            "SILVER_RATIO",
            "SPECIAL_CHARS",
            "SPEED_OF_LIGHT",
            "SQRT2",
            "SQRT3",
            "SQRT5",
            "STANDARD_ATMOSPHERE",
            "STANDARD_GRAVITY",
            "STEFAN_BOLTZMANN",
            "TAU",
            "VACUUM_PERMEABILITY",
            "VACUUM_PERMITTIVITY",
            "WIEN_DISPLACEMENT",
        ];
        for name in &expected {
            assert!(names.contains(name), "Missing constant: {name}");
        }
    }

    // ---------------------------------------------------------------
    // Constants::get_value — all ConstantValue variants
    // ---------------------------------------------------------------

    #[test]
    fn get_value_float_constants_table_driven() {
        let c = Constants::new();
        let float_names = [
            "APERY",
            "ATOMIC_MASS_UNIT",
            "AVOGADRO",
            "BOHR_RADIUS",
            "BOLTZMANN",
            "CATALAN",
            "CONDUCTANCE_QUANTUM",
            "COULOMB",
            "ELECTRON_MASS",
            "ELEMENTARY_CHARGE",
            "EULER",
            "FARADAY",
            "FINE_STRUCTURE",
            "FRAC_1_PI",
            "FRAC_1_SQRT_2",
            "FRAC_2_PI",
            "FRAC_2_SQRT_PI",
            "FRAC_PI_2",
            "FRAC_PI_3",
            "FRAC_PI_4",
            "FRAC_PI_6",
            "FRAC_PI_8",
            "GAMMA",
            "GAS_CONSTANT",
            "GLAISHER_KINKELIN",
            "GRAVITATIONAL_CONSTANT",
            "KHINCHIN",
            "LN_2",
            "LN_10",
            "LOG2_E",
            "LOG10_E",
            "MAGNETIC_FLUX_QUANTUM",
            "NEUTRON_MASS",
            "PHI",
            "PI",
            "PLANCK",
            "PLANCK_REDUCED",
            "PROTON_MASS",
            "RYDBERG",
            "SILVER_RATIO",
            "SPEED_OF_LIGHT",
            "SQRT2",
            "SQRT3",
            "SQRT5",
            "STANDARD_ATMOSPHERE",
            "STANDARD_GRAVITY",
            "STEFAN_BOLTZMANN",
            "TAU",
            "VACUUM_PERMEABILITY",
            "VACUUM_PERMITTIVITY",
            "WIEN_DISPLACEMENT",
        ];

        for name in &float_names {
            let value = c.get_value(name);
            assert!(
                value.is_some(),
                "get_value(\"{name}\") should return Some"
            );
            assert!(
                matches!(value.unwrap(), ConstantValue::Float(_)),
                "get_value(\"{name}\") should return Float variant"
            );
        }
    }

    #[test]
    fn get_value_hash_cost_returns_u32() {
        let c = Constants::new();
        if let Some(ConstantValue::U32(v)) = c.get_value("HASH_COST") {
            assert_eq!(v, HASH_COST);
        } else {
            panic!("Expected ConstantValue::U32");
        }
    }

    #[test]
    fn get_value_hash_length_returns_usize() {
        let c = Constants::new();
        if let Some(ConstantValue::Usize(v)) =
            c.get_value("HASH_LENGTH")
        {
            assert_eq!(v, HASH_LENGTH);
        } else {
            panic!("Expected ConstantValue::Usize");
        }
    }

    #[test]
    fn get_value_hash_algorithm_returns_string() {
        let c = Constants::new();
        if let Some(ConstantValue::String(v)) =
            c.get_value("HASH_ALGORITHM")
        {
            assert_eq!(v, HASH_ALGORITHM);
        } else {
            panic!("Expected ConstantValue::String");
        }
    }

    #[test]
    fn get_value_special_chars_returns_char_array() {
        let c = Constants::new();
        if let Some(ConstantValue::CharArray(arr)) =
            c.get_value("SPECIAL_CHARS")
        {
            assert_eq!(arr, SPECIAL_CHARS);
        } else {
            panic!("Expected ConstantValue::CharArray");
        }
    }

    #[test]
    fn get_value_nonexistent_returns_none() {
        let c = Constants::new();
        assert!(c.get_value("DOES_NOT_EXIST").is_none());
    }

    // ---------------------------------------------------------------
    // Constants::is_valid
    // ---------------------------------------------------------------

    #[test]
    fn is_valid_default_returns_true() {
        let c = Constants::new();
        assert!(c.is_valid());
    }

    // ---------------------------------------------------------------
    // Constant struct — derive trait coverage via retrieved instances
    // ---------------------------------------------------------------

    #[test]
    fn constant_clone_produces_equal() {
        let c = get("PI");
        let cloned = c.clone();
        assert_eq!(c, cloned);
    }

    #[test]
    fn constant_debug_contains_name() {
        let c = get("PI");
        let debug = format!("{:?}", c);
        assert!(debug.contains("PI"));
    }

    #[test]
    fn constant_eq_same_name() {
        let a = get("EULER");
        let b = get("EULER");
        assert_eq!(a, b);
    }

    #[test]
    fn constant_ne_different_name() {
        let a = get("EULER");
        let b = get("PI");
        assert_ne!(a, b);
    }

    #[test]
    fn constant_ord_sorts_by_name() {
        let apery = get("APERY");
        let euler = get("EULER");
        assert!(apery < euler);
    }

    #[test]
    fn constant_hash_consistent() {
        use std::collections::HashSet;
        let a = get("TAU");
        let b = get("TAU");
        let mut set = HashSet::new();
        let _ = set.insert(a);
        let _ = set.insert(b);
        assert_eq!(
            set.len(),
            1,
            "Equal constants should hash the same"
        );
    }

    #[test]
    fn constant_serialize_produces_valid_json() {
        let original = get("PI");
        let json = serde_json::to_string(&original).unwrap();
        let value: serde_json::Value =
            serde_json::from_str(&json).unwrap();
        assert_eq!(value["name"], "PI");
        assert!(!value["value"].as_str().unwrap().is_empty());
    }

    #[test]
    fn constant_deserialize_from_json() {
        // Use a leaked string to satisfy the 'static lifetime on name
        let json: &'static str = r#"{"name":"TEST","value":"42"}"#;
        let c: Constant = serde_json::from_str(json).unwrap();
        assert_eq!(c.name, "TEST");
        assert_eq!(c.value, "42");
    }

    // ---------------------------------------------------------------
    // Constants struct — Clone, Serialize, Debug
    // ---------------------------------------------------------------

    #[test]
    fn constants_clone_preserves_length() {
        let c = Constants::new();
        let cloned = c.clone();
        assert_eq!(c.constants().len(), cloned.constants().len());
    }

    #[test]
    fn constants_debug_is_not_empty() {
        let c = Constants::new();
        let debug = format!("{:?}", c);
        assert!(debug.contains("Constants"));
    }

    #[test]
    fn constants_serialize_produces_valid_json() {
        let c = Constants::new();
        let json = serde_json::to_string(&c).unwrap();
        assert!(json.contains("EULER"));
        assert!(json.contains("PI"));
    }

    // ---------------------------------------------------------------
    // ConstantValue — Clone, Debug, Serialize for each variant
    // ---------------------------------------------------------------

    #[test]
    fn constant_value_float_clone_and_debug() {
        let v = ConstantValue::Float(1.2345);
        let cloned = v.clone();
        let debug = format!("{:?}", cloned);
        assert!(debug.contains("1.2345"));
    }

    #[test]
    fn constant_value_string_clone_and_debug() {
        let v = ConstantValue::String("test".to_string());
        let cloned = v.clone();
        let debug = format!("{:?}", cloned);
        assert!(debug.contains("test"));
    }

    #[test]
    fn constant_value_u32_serialize() {
        let v = ConstantValue::U32(42);
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("42"));
    }

    #[test]
    fn constant_value_usize_serialize() {
        let v = ConstantValue::Usize(128);
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("128"));
    }

    #[test]
    fn constant_value_float_serialize() {
        let v = ConstantValue::Float(2.5);
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("2.5"));
    }

    #[test]
    fn constant_value_char_array_debug() {
        let v = ConstantValue::CharArray(SPECIAL_CHARS);
        let debug = format!("{:?}", v);
        assert!(debug.contains("CharArray"));
    }

    // ---------------------------------------------------------------
    // Compile-time constant values — table-driven verification
    // ---------------------------------------------------------------

    #[test]
    fn const_values_match_expected() {
        assert_eq!(APERY, 1.2020569031595942);
        assert_eq!(AVOGADRO, 6.02214076e23);
        assert_eq!(BOLTZMANN, 1.380649e-23);
        assert_eq!(CATALAN, 0.915_965_594_177_219);
        assert_eq!(COULOMB, 8.9875517923e9);
        assert_eq!(EULER, std::f64::consts::E);
        assert_eq!(FARADAY, 96485.33212);
        assert_eq!(GAMMA, 0.5772156649015329);
        assert_eq!(GAS_CONSTANT, 8.314462618);
        assert_eq!(GLAISHER_KINKELIN, 1.2824271291006226);
        assert_eq!(GRAVITATIONAL_CONSTANT, 6.67430e-11);
        assert_eq!(HASH_ALGORITHM, "Blake3");
        assert_eq!(HASH_COST, 8);
        assert_eq!(HASH_LENGTH, 32);
        assert_eq!(KHINCHIN, 2.6854520010653064);
        assert_eq!(PHI, (1.0 + SQRT5) / 2.0);
        assert_eq!(PI, std::f64::consts::PI);
        assert_eq!(PLANCK, 6.62607015e-34);
        assert_eq!(PLANCK_REDUCED, PLANCK / (2.0 * PI));
        assert_eq!(SILVER_RATIO, 1.0 + SQRT2);
        assert_eq!(SPEED_OF_LIGHT, 299_792_458.0);
        assert_eq!(SQRT2, std::f64::consts::SQRT_2);
        assert_eq!(SQRT3, 1.7320508075688772);
        assert_eq!(SQRT5, 2.236_067_977_499_79);
        assert_eq!(TAU, std::f64::consts::TAU);
        assert_eq!(VACUUM_PERMEABILITY, 1.25663706212e-6);
        assert_eq!(VACUUM_PERMITTIVITY, 8.8541878128e-12);
    }

    #[test]
    fn special_chars_has_expected_length() {
        assert_eq!(SPECIAL_CHARS.len(), 29);
    }

    // ---------------------------------------------------------------
    // New stdlib math constants — value verification
    // ---------------------------------------------------------------

    #[test]
    fn new_math_constants_match_std() {
        assert_eq!(LN_2, std::f64::consts::LN_2);
        assert_eq!(LN_10, std::f64::consts::LN_10);
        assert_eq!(LOG2_E, std::f64::consts::LOG2_E);
        assert_eq!(LOG10_E, std::f64::consts::LOG10_E);
        assert_eq!(FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2);
        assert_eq!(FRAC_1_PI, std::f64::consts::FRAC_1_PI);
        assert_eq!(FRAC_2_PI, std::f64::consts::FRAC_2_PI);
        assert_eq!(FRAC_2_SQRT_PI, std::f64::consts::FRAC_2_SQRT_PI);
        assert_eq!(FRAC_PI_2, std::f64::consts::FRAC_PI_2);
        assert_eq!(FRAC_PI_3, std::f64::consts::FRAC_PI_3);
        assert_eq!(FRAC_PI_4, std::f64::consts::FRAC_PI_4);
        assert_eq!(FRAC_PI_6, std::f64::consts::FRAC_PI_6);
        assert_eq!(FRAC_PI_8, std::f64::consts::FRAC_PI_8);
    }

    // ---------------------------------------------------------------
    // New CODATA physical constants — value verification
    // ---------------------------------------------------------------

    #[test]
    fn new_physical_constants_values() {
        assert_eq!(ELEMENTARY_CHARGE, 1.602_176_634e-19);
        assert_eq!(ELECTRON_MASS, 9.109_383_701_5e-31);
        assert_eq!(PROTON_MASS, 1.672_621_923_69e-27);
        assert_eq!(NEUTRON_MASS, 1.674_927_498_04e-27);
        assert_eq!(STEFAN_BOLTZMANN, 5.670_374_419e-8);
        assert_eq!(WIEN_DISPLACEMENT, 2.897_771_955e-3);
        assert_eq!(STANDARD_GRAVITY, 9.806_65);
        assert_eq!(STANDARD_ATMOSPHERE, 101_325.0);
        assert_eq!(ATOMIC_MASS_UNIT, 1.660_539_066_60e-27);
        assert_eq!(BOHR_RADIUS, 5.291_772_109_03e-11);
        assert_eq!(FINE_STRUCTURE, 7.297_352_569_3e-3);
        assert_eq!(RYDBERG, 10_973_731.568_160);
        assert_eq!(MAGNETIC_FLUX_QUANTUM, 2.067_833_848e-15);
        assert_eq!(CONDUCTANCE_QUANTUM, 7.748_091_729e-5);
    }

    // ---------------------------------------------------------------
    // Derived constant relationships
    // ---------------------------------------------------------------

    #[test]
    fn phi_equals_golden_ratio_formula() {
        let expected = (1.0_f64 + 5.0_f64.sqrt()) / 2.0;
        assert!((PHI - expected).abs() < 1e-15);
    }

    #[test]
    fn tau_equals_two_pi() {
        assert!((TAU - 2.0 * PI).abs() < 1e-15);
    }

    #[test]
    fn silver_ratio_equals_one_plus_sqrt2() {
        assert!((SILVER_RATIO - (1.0 + SQRT2)).abs() < 1e-15);
    }

    #[test]
    fn planck_reduced_equals_planck_over_two_pi() {
        assert!((PLANCK_REDUCED - PLANCK / (2.0 * PI)).abs() < 1e-50);
    }

    // --- New math identity checks ---

    #[test]
    fn ln2_exp_equals_two() {
        assert!((LN_2.exp() - 2.0).abs() < 1e-14);
    }

    #[test]
    fn ln10_exp_equals_ten() {
        assert!((LN_10.exp() - 10.0).abs() < 1e-13);
    }

    #[test]
    fn log2e_times_ln2_equals_one() {
        assert!((LOG2_E * LN_2 - 1.0).abs() < 1e-15);
    }

    #[test]
    fn log10e_times_ln10_equals_one() {
        assert!((LOG10_E * LN_10 - 1.0).abs() < 1e-15);
    }

    #[test]
    fn frac_1_sqrt2_squared_equals_half() {
        assert!((FRAC_1_SQRT_2 * FRAC_1_SQRT_2 - 0.5).abs() < 1e-15);
    }

    #[test]
    fn frac_pi_fractions_consistent() {
        assert!((FRAC_PI_2 - PI / 2.0).abs() < 1e-15);
        assert!((FRAC_PI_3 - PI / 3.0).abs() < 1e-15);
        assert!((FRAC_PI_4 - PI / 4.0).abs() < 1e-15);
        assert!((FRAC_PI_6 - PI / 6.0).abs() < 1e-15);
        assert!((FRAC_PI_8 - PI / 8.0).abs() < 1e-15);
    }

    #[test]
    fn frac_1_pi_times_pi_equals_one() {
        assert!((FRAC_1_PI * PI - 1.0).abs() < 1e-15);
    }

    // --- New physics identity checks ---

    #[test]
    fn magnetic_flux_quantum_identity() {
        // Φ_0 = h / (2e)
        let computed = PLANCK / (2.0 * ELEMENTARY_CHARGE);
        assert!(
            (MAGNETIC_FLUX_QUANTUM - computed).abs() / computed < 1e-6
        );
    }

    #[test]
    fn conductance_quantum_identity() {
        // G_0 = 2e² / h
        let computed =
            2.0 * ELEMENTARY_CHARGE * ELEMENTARY_CHARGE / PLANCK;
        assert!(
            (CONDUCTANCE_QUANTUM - computed).abs() / computed < 1e-6
        );
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn proton_heavier_than_electron() {
        assert!(PROTON_MASS > ELECTRON_MASS);
        assert!(PROTON_MASS / ELECTRON_MASS > 1836.0);
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn neutron_heavier_than_proton() {
        assert!(NEUTRON_MASS > PROTON_MASS);
    }

    // ---------------------------------------------------------------
    // ConstantValue Display
    // ---------------------------------------------------------------

    #[test]
    fn constant_value_display_float() {
        let v = ConstantValue::Float(2.5);
        assert_eq!(format!("{v}"), "2.5");
    }

    #[test]
    fn constant_value_display_string() {
        let v = ConstantValue::String("Blake3".to_string());
        assert_eq!(format!("{v}"), "Blake3");
    }

    #[test]
    fn constant_value_display_u32() {
        let v = ConstantValue::U32(8);
        assert_eq!(format!("{v}"), "8");
    }

    #[test]
    fn constant_value_display_usize() {
        let v = ConstantValue::Usize(32);
        assert_eq!(format!("{v}"), "32");
    }

    #[test]
    fn constant_value_display_char_array() {
        let v = ConstantValue::CharArray(&['a', 'b']);
        assert_eq!(format!("{v}"), "ab");
    }

    // ---------------------------------------------------------------
    // CONSTANTS_TABLE with Category
    // ---------------------------------------------------------------

    #[test]
    fn constants_table_has_106_float_entries() {
        use cmn::constants::CONSTANTS_TABLE;
        assert_eq!(CONSTANTS_TABLE.len(), 106);
    }

    #[test]
    fn constants_table_category_filter() {
        use cmn::constants::{Category, CONSTANTS_TABLE};
        let math: Vec<_> = CONSTANTS_TABLE
            .iter()
            .filter(|(_, _, c)| *c == Category::Mathematical)
            .collect();
        let phys: Vec<_> = CONSTANTS_TABLE
            .iter()
            .filter(|(_, _, c)| *c == Category::Physical)
            .collect();
        assert!(math.len() > 20);
        assert!(phys.len() > 20);
    }
}
