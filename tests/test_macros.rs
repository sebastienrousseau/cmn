// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(test)]
mod tests {
    use cmn::Common;
    use cmn::{
        cmn_assert, cmn_constants, cmn_contains, cmn_in_range, cmn_join,
        cmn_map, cmn_max, cmn_min, cmn_parse, cmn_print, cmn_print_vec,
        cmn_split, cmn_to_num, cmn_vec, constants::*,
    };

    // ===============================================================
    // cmn_assert!
    // ===============================================================

    #[test]
    fn cmn_assert_true_does_not_panic() {
        cmn_assert!(true);
    }

    #[test]
    #[should_panic(expected = "Assertion failed!")]
    fn cmn_assert_false_panics() {
        cmn_assert!(false);
    }

    #[test]
    fn cmn_assert_expression_true() {
        cmn_assert!(1 + 1 == 2);
    }

    // ===============================================================
    // cmn_contains!
    // ===============================================================

    #[test]
    fn cmn_contains_present_returns_true() {
        assert!(cmn_contains!("Hello, World!", "World"));
    }

    #[test]
    fn cmn_contains_absent_returns_false() {
        assert!(!cmn_contains!("Hello", "xyz"));
    }

    #[test]
    fn cmn_contains_empty_substring_returns_true() {
        assert!(cmn_contains!("anything", ""));
    }

    #[test]
    fn cmn_contains_empty_haystack_returns_false() {
        assert!(!cmn_contains!("", "something"));
    }

    #[test]
    fn cmn_contains_both_empty_returns_true() {
        assert!(cmn_contains!("", ""));
    }

    // ===============================================================
    // cmn_in_range!
    // ===============================================================

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn cmn_in_range_within_returns_true() {
        debug_assert!(cmn_in_range!(5, 0, 10));
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn cmn_in_range_below_returns_false() {
        debug_assert!(!cmn_in_range!(-1, 0, 10));
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn cmn_in_range_above_returns_false() {
        debug_assert!(!cmn_in_range!(11, 0, 10));
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn cmn_in_range_at_min_boundary_returns_true() {
        debug_assert!(cmn_in_range!(0, 0, 10));
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn cmn_in_range_at_max_boundary_returns_true() {
        debug_assert!(cmn_in_range!(10, 0, 10));
    }

    #[test]
    #[allow(clippy::assertions_on_constants, clippy::double_comparisons)]
    fn cmn_in_range_equal_min_max_at_value_returns_true() {
        debug_assert!(cmn_in_range!(5, 5, 5));
    }

    #[test]
    #[allow(clippy::assertions_on_constants, clippy::double_comparisons)]
    fn cmn_in_range_equal_min_max_not_at_value_returns_false() {
        debug_assert!(!cmn_in_range!(6, 5, 5));
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn cmn_in_range_float_values() {
        debug_assert!(cmn_in_range!(0.5, 0.0, 1.0));
        debug_assert!(!cmn_in_range!(1.5, 0.0, 1.0));
    }

    // ===============================================================
    // cmn_join!
    // ===============================================================

    #[test]
    fn cmn_join_multiple_strings() {
        let s = cmn_join!("Hello", " ", "World");
        assert_eq!(s, "Hello World");
    }

    #[test]
    fn cmn_join_single_string() {
        let s = cmn_join!("alone");
        assert_eq!(s, "alone");
    }

    #[test]
    fn cmn_join_empty_strings() {
        let s = cmn_join!("", "", "");
        assert_eq!(s, "");
    }

    // ===============================================================
    // cmn_map!
    // ===============================================================

    #[test]
    fn cmn_map_creates_correct_entries() {
        let map = cmn_map!("a" => 1, "b" => 2, "c" => 3);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
        assert_eq!(map.get("c"), Some(&3));
    }

    #[test]
    fn cmn_map_missing_key_returns_none() {
        let map = cmn_map!("x" => 42);
        assert_eq!(map.get("y"), None);
    }

    // ===============================================================
    // cmn_max!
    // ===============================================================

    #[test]
    fn cmn_max_multiple_values() {
        assert_eq!(cmn_max!(10, 20, 30), 30);
    }

    #[test]
    fn cmn_max_single_value() {
        assert_eq!(cmn_max!(42), 42);
    }

    #[test]
    fn cmn_max_negative_values() {
        assert_eq!(cmn_max!(-10, -20, -5), -5);
    }

    #[test]
    fn cmn_max_equal_values() {
        assert_eq!(cmn_max!(7, 7, 7), 7);
    }

    // ===============================================================
    // cmn_min!
    // ===============================================================

    #[test]
    fn cmn_min_multiple_values() {
        assert_eq!(cmn_min!(10, 20, 30), 10);
    }

    #[test]
    fn cmn_min_single_value() {
        assert_eq!(cmn_min!(42), 42);
    }

    #[test]
    fn cmn_min_negative_values() {
        assert_eq!(cmn_min!(-10, -20, -5), -20);
    }

    #[test]
    fn cmn_min_equal_values() {
        assert_eq!(cmn_min!(7, 7, 7), 7);
    }

    // ===============================================================
    // cmn_parse!
    // ===============================================================

    #[test]
    fn cmn_parse_valid_json_with_words() {
        let json = r#"{"words": ["foo", "bar", "baz"]}"#;
        let common = cmn_parse!(json).unwrap();
        let list = common.words().words_list();
        assert_eq!(list, vec!["bar", "baz", "foo"]);
    }

    #[test]
    fn cmn_parse_empty_object() {
        let result = cmn_parse!("{}");
        assert!(result.is_ok());
    }

    #[test]
    fn cmn_parse_invalid_json() {
        let result = cmn_parse!("not json");
        assert!(result.is_err());
    }

    // ===============================================================
    // cmn_print! / cmn_print_vec!
    // ===============================================================

    #[test]
    fn cmn_print_does_not_panic() {
        cmn_print!("Hello, World!");
    }

    #[test]
    fn cmn_print_number() {
        cmn_print!(42);
    }

    #[test]
    fn cmn_print_vec_does_not_panic() {
        cmn_print_vec!(&[1, 2, 3]);
    }

    #[test]
    fn cmn_print_vec_empty() {
        let empty: &[i32] = &[];
        cmn_print_vec!(empty);
    }

    // ===============================================================
    // cmn_split!
    // ===============================================================

    #[test]
    fn cmn_split_whitespace_separated() {
        let v = cmn_split!("Hello World Rust");
        assert_eq!(v, vec!["Hello", "World", "Rust"]);
    }

    #[test]
    fn cmn_split_single_word() {
        let v = cmn_split!("single");
        assert_eq!(v, vec!["single"]);
    }

    #[test]
    fn cmn_split_empty_string() {
        let v = cmn_split!("");
        assert!(v.is_empty());
    }

    #[test]
    fn cmn_split_multiple_spaces() {
        let v = cmn_split!("  hello   world  ");
        assert_eq!(v, vec!["hello", "world"]);
    }

    // ===============================================================
    // cmn_to_num!
    // ===============================================================

    #[test]
    fn cmn_to_num_valid_integer() {
        assert_eq!(cmn_to_num!("42"), 42.0);
    }

    #[test]
    fn cmn_to_num_valid_float() {
        assert_eq!(cmn_to_num!("1.23"), 1.23);
    }

    #[test]
    fn cmn_to_num_invalid_returns_zero() {
        assert_eq!(cmn_to_num!("not_a_number"), 0.0);
    }

    #[test]
    fn cmn_to_num_empty_string_returns_zero() {
        assert_eq!(cmn_to_num!(""), 0.0);
    }

    #[test]
    fn cmn_to_num_negative() {
        assert_eq!(cmn_to_num!("-7.5"), -7.5);
    }

    // ===============================================================
    // cmn_vec!
    // ===============================================================

    #[test]
    fn cmn_vec_creates_vector() {
        let v = cmn_vec!(1, 2, 3);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn cmn_vec_single_element() {
        let v = cmn_vec!(42);
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn cmn_vec_string_elements() {
        let v = cmn_vec!("a", "b", "c");
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    // ===============================================================
    // cmn_constants! macro
    // ===============================================================

    #[test]
    fn cmn_constants_macro_defines_values() {
        cmn_constants! {
            TEST_PI = std::f64::consts::PI,
            TEST_E = std::f64::consts::E
        }
        assert_eq!(TEST_PI, std::f64::consts::PI);
        assert_eq!(TEST_E, std::f64::consts::E);
    }

    // ===============================================================
    // cmn! macro (top-level parse)
    // ===============================================================

    #[test]
    fn cmn_macro_valid_json() {
        let result = cmn::cmn!(r#"{"key": "value"}"#);
        assert!(result.is_ok());
    }

    #[test]
    fn cmn_macro_invalid_json() {
        let result = cmn::cmn!("bad");
        assert!(result.is_err());
    }

    // ===============================================================
    // Constant values — spot checks via the constants module
    // ===============================================================

    #[test]
    fn all_constant_values_are_correct() {
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
        assert_eq!(PI, std::f64::consts::PI);
        assert_eq!(PLANCK, 6.62607015e-34);
        assert_eq!(SPEED_OF_LIGHT, 299_792_458.0);
        assert_eq!(SQRT2, std::f64::consts::SQRT_2);
        assert_eq!(SQRT3, 1.7320508075688772);
        assert_eq!(SQRT5, 2.236_067_977_499_79);
        assert_eq!(TAU, std::f64::consts::TAU);
        assert_eq!(VACUUM_PERMEABILITY, 1.25663706212e-6);
        assert_eq!(VACUUM_PERMITTIVITY, 8.8541878128e-12);
    }
}
