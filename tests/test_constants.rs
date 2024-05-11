// Copyright © 2023 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(test)]
mod tests {
    use cmn::constants::{ConstantValue, Constants};

    #[test]
    fn test_constant() {
        let new_constant = Constants::new();
        let constant = new_constant.constant("EULER");
        assert!(constant.is_some());
        assert_eq!(constant.unwrap().name, "EULER");

        let constant = new_constant.constant("FAKE_CONSTANT");
        assert!(constant.is_none());
    }

    #[test]
    fn test_constants() {
        let new_constant = Constants::new();
        let constants = new_constant.constants();
        assert_eq!(constants.len(), 28);

        let names =
            constants.iter().map(|c| c.name).collect::<Vec<_>>();
        assert!(names.contains(&"APERY"));
        assert!(names.contains(&"AVOGADRO"));
        assert!(names.contains(&"BOLTZMANN"));
        assert!(names.contains(&"CATALAN"));
        assert!(names.contains(&"COULOMB"));
        assert!(names.contains(&"EULER"));
        assert!(names.contains(&"FARADAY"));
        assert!(names.contains(&"GAMMA"));
        assert!(names.contains(&"GAS_CONSTANT"));
        assert!(names.contains(&"GLAISHER_KINKELIN"));
        assert!(names.contains(&"GRAVITATIONAL_CONSTANT"));
        assert!(names.contains(&"HASH_ALGORITHM"));
        assert!(names.contains(&"HASH_COST"));
        assert!(names.contains(&"HASH_LENGTH"));
        assert!(names.contains(&"KHINCHIN"));
        assert!(names.contains(&"PHI"));
        assert!(names.contains(&"PI"));
        assert!(names.contains(&"PLANCK"));
        assert!(names.contains(&"PLANCK_REDUCED"));
        assert!(names.contains(&"SILVER_RATIO"));
        assert!(names.contains(&"SPEED_OF_LIGHT"));
        assert!(names.contains(&"SPECIAL_CHARS"));
        assert!(names.contains(&"SQRT2"));
        assert!(names.contains(&"SQRT3"));
        assert!(names.contains(&"SQRT5"));
        assert!(names.contains(&"TAU"));
        assert!(names.contains(&"VACUUM_PERMEABILITY"));
        assert!(names.contains(&"VACUUM_PERMITTIVITY"));
    }

    #[test]
    fn test_new() {
        let new_constant = Constants::new();
        let constants = new_constant.constants();
        assert!(!constants.is_empty());
    }

    #[test]
    fn test_default() {
        let default_constant = Constants::default();
        let constants = default_constant.constants();
        assert!(!constants.is_empty());
    }

    #[test]
    fn test_get_value() {
        let constants = Constants::new();

        // Test getting a float value
        let value = constants.get_value("EULER");
        assert!(value.is_some());
        if let Some(ConstantValue::Float(float_value)) = value {
            assert!((float_value - std::f64::consts::E).abs() < 1e-10);
        } else {
            panic!("Expected a float value");
        }

        // Test getting a string value
        let value = constants.get_value("HASH_ALGORITHM");
        assert!(value.is_some());
        if let Some(ConstantValue::String(string_value)) = value {
            assert_eq!(string_value, "Blake3");
        } else {
            panic!("Expected a string value");
        }

        // Test getting a u32 value
        let value = constants.get_value("HASH_COST");
        assert!(value.is_some());
        if let Some(ConstantValue::U32(u32_value)) = value {
            assert_eq!(u32_value, 8);
        } else {
            panic!("Expected a u32 value");
        }

        // Test getting a usize value
        let value = constants.get_value("HASH_LENGTH");
        assert!(value.is_some());
        if let Some(ConstantValue::Usize(usize_value)) = value {
            assert_eq!(usize_value, 32);
        } else {
            panic!("Expected a usize value");
        }

        // Test getting a char array value
        let value = constants.get_value("SPECIAL_CHARS");
        assert!(value.is_some());
        if let Some(ConstantValue::CharArray(char_array)) = value {
            assert_eq!(
                char_array,
                &[
                    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')',
                    '_', '+', '=', '[', ']', '{', '}', '|', ';', ':',
                    '"', '<', '>', ',', '.', '?', '/', '~', '`'
                ]
            );
        } else {
            panic!("Expected a char array value");
        }

        // Test getting a non-existent constant value
        let value = constants.get_value("NON_EXISTENT");
        assert!(value.is_none());
    }
}
