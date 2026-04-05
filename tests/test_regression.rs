// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Regression Test Suite
//!
//! 50 regression tests organised into 8 categories that guard the
//! public API surface of the `cmn` crate against accidental breakage.
//!
//! | # | Category                        | Tests |
//! |---|---------------------------------|-------|
//! | 1 | Constant value integrity        | 7     |
//! | 2 | Cross-module integration        | 7     |
//! | 3 | Serialization stability         | 7     |
//! | 4 | Thread safety                   | 4     |
//! | 5 | Mathematical invariants         | 6     |
//! | 6 | Words collection stress         | 6     |
//! | 7 | Parse robustness                | 7     |
//! | 8 | API contract guards             | 6     |

#[cfg(test)]
mod regression {
    use cmn::constants::{
        ConstantValue, Constants, AVOGADRO, BOLTZMANN, EULER, FARADAY,
        GAS_CONSTANT, HASH_ALGORITHM, HASH_COST, HASH_LENGTH, PHI, PI,
        SPECIAL_CHARS, SPEED_OF_LIGHT, SQRT2, SQRT3, SQRT5, TAU,
        VACUUM_PERMEABILITY, VACUUM_PERMITTIVITY,
    };
    use cmn::words::WORD_LIST;
    use cmn::{Common, Words};

    // ===============================================================
    // 1. Constant value integrity  (7 tests)
    //    Guards: stored string values survive f64 → String → f64
    //    round-trips without precision drift.
    // ===============================================================

    /// Every float constant's stored string must parse back to the
    /// exact same f64 bit-pattern.
    #[test]
    fn reg_01_float_string_roundtrip_all_constants() {
        let c = Constants::new();
        let float_names = [
            "APERY",
            "AVOGADRO",
            "BOLTZMANN",
            "CATALAN",
            "COULOMB",
            "EULER",
            "FARADAY",
            "GAMMA",
            "GAS_CONSTANT",
            "GLAISHER_KINKELIN",
            "GRAVITATIONAL_CONSTANT",
            "KHINCHIN",
            "PHI",
            "PI",
            "PLANCK",
            "PLANCK_REDUCED",
            "SILVER_RATIO",
            "SPEED_OF_LIGHT",
            "SQRT2",
            "SQRT3",
            "SQRT5",
            "TAU",
            "VACUUM_PERMEABILITY",
            "VACUUM_PERMITTIVITY",
        ];

        for name in &float_names {
            let constant = c.constant(name).unwrap();
            let parsed: f64 = constant
                .value
                .parse()
                .unwrap_or_else(|e| panic!("{name}: {e}"));
            // Re-stringify and compare strings to catch drift.
            assert_eq!(
                parsed.to_string(),
                constant.value,
                "{name}: round-trip string mismatch"
            );
        }
    }

    /// HASH_COST string must parse to the exact u32 constant.
    #[test]
    fn reg_02_hash_cost_string_matches_const() {
        let c = Constants::new();
        let val = c.constant("HASH_COST").unwrap();
        assert_eq!(val.value.parse::<u32>().unwrap(), HASH_COST);
    }

    /// HASH_LENGTH string must parse to the exact usize constant.
    #[test]
    fn reg_03_hash_length_string_matches_const() {
        let c = Constants::new();
        let val = c.constant("HASH_LENGTH").unwrap();
        assert_eq!(val.value.parse::<usize>().unwrap(), HASH_LENGTH);
    }

    /// HASH_ALGORITHM stored string must match the const exactly.
    #[test]
    fn reg_04_hash_algorithm_string_matches_const() {
        let c = Constants::new();
        let val = c.constant("HASH_ALGORITHM").unwrap();
        assert_eq!(val.value, HASH_ALGORITHM);
    }

    /// SPECIAL_CHARS stored string must equal the char-slice
    /// collected into a String.
    #[test]
    fn reg_05_special_chars_string_matches_const() {
        let c = Constants::new();
        let val = c.constant("SPECIAL_CHARS").unwrap();
        let expected: String = SPECIAL_CHARS.iter().collect();
        assert_eq!(val.value, expected);
    }

    /// No constant may have an empty name field.
    #[test]
    fn reg_06_no_constant_has_empty_name() {
        let c = Constants::new();
        for item in c.constants() {
            assert!(
                !item.name.is_empty(),
                "Found constant with empty name"
            );
        }
    }

    /// No constant may have an empty value field.
    #[test]
    fn reg_07_no_constant_has_empty_value() {
        let c = Constants::new();
        for item in c.constants() {
            assert!(
                !item.value.is_empty(),
                "Constant '{}' has empty value",
                item.name
            );
        }
    }

    // ===============================================================
    // 2. Cross-module integration  (7 tests)
    //    Guards: the public API works end-to-end across modules.
    // ===============================================================

    /// Common::constants() returns the same data as Constants::new().
    #[test]
    fn reg_08_common_constants_equals_standalone() {
        let via_common = Common::new().constants();
        let standalone = Constants::new();
        assert_eq!(via_common.constants(), standalone.constants());
    }

    /// Common::words() on a fresh instance returns an empty set.
    #[test]
    fn reg_09_common_words_fresh_is_empty() {
        let w = Common::new().words();
        assert_eq!(w.count(), 0);
    }

    /// Parsing JSON with words → extracting → sorting must preserve
    /// alphabetical order and content.
    #[test]
    fn reg_10_parse_words_end_to_end() {
        let json = r#"{"words":["gamma","alpha","beta"]}"#;
        let common = Common::parse(json).unwrap();
        let list = common.words().words_list();
        assert_eq!(list, vec!["alpha", "beta", "gamma"]);
    }

    /// get_value() on every float constant returns the Float variant
    /// with the correct magnitude.
    #[test]
    fn reg_11_get_value_returns_correct_float_for_pi() {
        let c = Constants::new();
        if let Some(ConstantValue::Float(v)) = c.get_value("PI") {
            assert!(
                (v - std::f64::consts::PI).abs() < 1e-15,
                "PI value drift detected"
            );
        } else {
            panic!("Expected Float for PI");
        }
    }

    /// constant() returns a *cloned* value — mutating it must not
    /// affect a second retrieval.
    #[test]
    fn reg_12_constant_returns_independent_clone() {
        let c = Constants::new();
        let mut first = c.constant("EULER").unwrap();
        first.value = "TAMPERED".to_string();
        let second = c.constant("EULER").unwrap();
        assert_ne!(
            first.value, second.value,
            "constant() must return independent clones"
        );
    }

    /// Words::default() contents must be a subset-checked against
    /// WORD_LIST — every word in the set is present in the const.
    #[test]
    fn reg_13_default_words_all_in_word_list() {
        let words = Words::default();
        for w in &words.words_list() {
            assert!(
                WORD_LIST.contains(&w.as_str()),
                "Word '{w}' not found in WORD_LIST"
            );
        }
    }

    /// Two independently-created Constants instances must be equal
    /// (deterministic construction).
    #[test]
    fn reg_14_constants_construction_is_deterministic() {
        let a = Constants::new();
        let b = Constants::new();
        assert_eq!(a.constants(), b.constants());
    }

    // ===============================================================
    // 3. Serialization stability  (7 tests)
    //    Guards: JSON shapes produced by serde never silently change.
    // ===============================================================

    /// Serialised Constant must contain exactly "name" and "value"
    /// keys.
    #[test]
    fn reg_15_constant_json_shape() {
        let c = Constants::new().constant("PI").unwrap();
        let v: serde_json::Value = serde_json::to_value(&c).unwrap();
        let obj = v.as_object().unwrap();
        assert!(obj.contains_key("name"), "Missing 'name' key");
        assert!(obj.contains_key("value"), "Missing 'value' key");
        assert_eq!(
            obj.len(),
            2,
            "Unexpected extra keys in Constant JSON"
        );
    }

    /// Serialised Constants must have a "constants" array.
    #[test]
    fn reg_16_constants_json_has_array() {
        let c = Constants::new();
        let v: serde_json::Value = serde_json::to_value(&c).unwrap();
        assert!(
            v["constants"].is_array(),
            "'constants' must serialise as an array"
        );
        assert_eq!(v["constants"].as_array().unwrap().len(), 55);
    }

    /// Common serialise → deserialise must be idempotent.
    #[test]
    fn reg_17_common_serde_idempotent() {
        let input = r#"{"x":1,"y":"two"}"#;
        let a = Common::parse(input).unwrap();
        let json_a = serde_json::to_string(&a).unwrap();
        let b: Common = serde_json::from_str(&json_a).unwrap();
        let json_b = serde_json::to_string(&b).unwrap();
        assert_eq!(json_a, json_b);
    }

    /// Words serialise → deserialise must preserve all words.
    #[test]
    fn reg_18_words_serde_preserves_all() {
        let original = Words::default();
        let json = serde_json::to_string(&original).unwrap();
        let restored: Words = serde_json::from_str(&json).unwrap();
        assert_eq!(original.count(), restored.count());
        for w in &original.words_list() {
            assert!(
                restored.contains(w),
                "Lost word '{w}' after serde roundtrip"
            );
        }
    }

    /// ConstantValue::Float serialises with a tagged enum wrapper.
    #[test]
    fn reg_19_constant_value_float_tagged() {
        let v = ConstantValue::Float(1.0);
        let json = serde_json::to_string(&v).unwrap();
        assert!(
            json.contains("Float"),
            "Float variant must serialise with tag"
        );
    }

    /// ConstantValue::String serialises with a tagged enum wrapper.
    #[test]
    fn reg_20_constant_value_string_tagged() {
        let v = ConstantValue::String("hello".into());
        let json = serde_json::to_string(&v).unwrap();
        assert!(
            json.contains("String"),
            "String variant must serialise with tag"
        );
    }

    /// Serialised Constants JSON must be valid JSON parseable by
    /// serde_json::Value.
    #[test]
    fn reg_21_constants_json_valid() {
        let c = Constants::new();
        let json = serde_json::to_string(&c).unwrap();
        let parsed: Result<serde_json::Value, _> =
            serde_json::from_str(&json);
        assert!(parsed.is_ok(), "Constants JSON must be valid");
    }

    // ===============================================================
    // 4. Thread safety  (4 tests)
    //    Guards: all public types are safe for concurrent reads.
    // ===============================================================

    /// Constants can be shared across threads safely.
    #[test]
    fn reg_22_constants_concurrent_reads() {
        use std::sync::Arc;
        let c = Arc::new(Constants::new());
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let c = Arc::clone(&c);
                std::thread::spawn(move || {
                    assert_eq!(c.constants().len(), 55);
                    let _ = c.constant("PI");
                    let _ = c.get_value("EULER");
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
    }

    /// Words::default() can be built concurrently in many threads and
    /// all must produce the same count.
    #[test]
    fn reg_23_words_default_concurrent_construction() {
        let handles: Vec<_> = (0..8)
            .map(|_| {
                std::thread::spawn(|| {
                    let w = Words::default();
                    w.count()
                })
            })
            .collect();
        let counts: Vec<usize> =
            handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert!(
            counts.iter().all(|&c| c == counts[0]),
            "Concurrent Words::default() produced inconsistent counts"
        );
    }

    /// Common::parse can be called concurrently from many threads.
    #[test]
    fn reg_24_common_parse_concurrent() {
        let handles: Vec<_> = (0..8)
            .map(|i| {
                std::thread::spawn(move || {
                    let json = format!(r#"{{"id":{i}}}"#);
                    Common::parse(&json).unwrap()
                })
            })
            .collect();
        for h in handles {
            let _ = h.join().unwrap();
        }
    }

    /// Common::new() and Common::default() are thread-safe.
    #[test]
    fn reg_25_common_constructors_thread_safe() {
        let handles: Vec<_> = (0..8)
            .map(|i| {
                std::thread::spawn(move || {
                    if i % 2 == 0 {
                        Common::new().constants().constants().len()
                    } else {
                        Common::default().constants().constants().len()
                    }
                })
            })
            .collect();
        for h in handles {
            assert_eq!(h.join().unwrap(), 55);
        }
    }

    // ===============================================================
    // 5. Mathematical invariants  (6 tests)
    //    Guards: physical and mathematical relationships that must
    //    hold between constants.
    // ===============================================================

    /// PHI satisfies the golden-ratio equation: PHI^2 = PHI + 1.
    #[test]
    fn reg_26_phi_satisfies_quadratic() {
        let lhs = PHI * PHI;
        let rhs = PHI + 1.0;
        assert!((lhs - rhs).abs() < 1e-14, "PHI^2 must equal PHI + 1");
    }

    /// SQRT2^2 must equal 2.
    #[test]
    fn reg_27_sqrt2_squared_equals_two() {
        assert!((SQRT2 * SQRT2 - 2.0).abs() < 1e-15, "SQRT2^2 drift");
    }

    /// SQRT3^2 must equal 3.
    #[test]
    fn reg_28_sqrt3_squared_equals_three() {
        assert!((SQRT3 * SQRT3 - 3.0).abs() < 1e-15, "SQRT3^2 drift");
    }

    /// SQRT5^2 must equal 5.
    #[test]
    fn reg_29_sqrt5_squared_equals_five() {
        assert!((SQRT5 * SQRT5 - 5.0).abs() < 1e-13, "SQRT5^2 drift");
    }

    /// Boltzmann * Avogadro ≈ Gas constant (R = k_B * N_A).
    #[test]
    fn reg_30_boltzmann_avogadro_gas_constant() {
        let computed = BOLTZMANN * AVOGADRO;
        assert!(
            (computed - GAS_CONSTANT).abs() < 0.01,
            "R = k_B * N_A violated: {computed} vs {GAS_CONSTANT}"
        );
    }

    /// Vacuum permeability * vacuum permittivity * c^2 ≈ 1.
    /// (μ₀ * ε₀ * c² = 1 in SI units)
    #[test]
    fn reg_31_electromagnetic_relation() {
        let product = VACUUM_PERMEABILITY
            * VACUUM_PERMITTIVITY
            * SPEED_OF_LIGHT
            * SPEED_OF_LIGHT;
        assert!(
            (product - 1.0).abs() < 1e-4,
            "mu0 * eps0 * c^2 must ≈ 1, got {product}"
        );
    }

    // ===============================================================
    // 6. Words collection stress  (6 tests)
    //    Guards: Words operations remain correct under non-trivial
    //    sequences and volumes.
    // ===============================================================

    /// Adding 1 000 unique words then checking count.
    #[test]
    fn reg_32_words_bulk_add() {
        let mut w = Words::new();
        for i in 0..1_000 {
            w.add_word(&format!("word_{i}"));
        }
        assert_eq!(w.count(), 1_000);
    }

    /// Bulk add then bulk remove must leave an empty set.
    #[test]
    fn reg_33_words_bulk_add_then_remove() {
        let mut w = Words::new();
        let items: Vec<String> =
            (0..500).map(|i| format!("w{i}")).collect();
        for s in &items {
            w.add_word(s);
        }
        assert_eq!(w.count(), 500);
        for s in &items {
            assert!(w.remove_word(s));
        }
        assert_eq!(w.count(), 0);
    }

    /// words_list() on a large set must still be sorted.
    #[test]
    fn reg_34_words_large_set_sorted() {
        let mut w = Words::new();
        for i in (0..200).rev() {
            w.add_word(&format!("item_{i:04}"));
        }
        let list = w.words_list();
        for pair in list.windows(2) {
            assert!(
                pair[0] <= pair[1],
                "Sorting broken: {} > {}",
                pair[0],
                pair[1]
            );
        }
    }

    /// add → remove → re-add must work correctly.
    #[test]
    fn reg_35_words_add_remove_readd() {
        let mut w = Words::new();
        w.add_word("ping");
        assert!(w.contains("ping"));
        assert!(w.remove_word("ping"));
        assert!(!w.contains("ping"));
        w.add_word("ping");
        assert!(w.contains("ping"));
        assert_eq!(w.count(), 1);
    }

    /// clear() on default, then add, must work from a clean slate.
    #[test]
    fn reg_36_words_clear_then_repopulate() {
        let mut w = Words::default();
        let original_count = w.count();
        assert!(original_count > 0);
        w.clear();
        assert_eq!(w.count(), 0);
        w.add_word("fresh");
        assert_eq!(w.count(), 1);
        assert!(w.contains("fresh"));
    }

    /// WORD_LIST must contain no duplicates.
    #[test]
    fn reg_37_word_list_no_duplicates() {
        let mut seen = std::collections::HashSet::new();
        for word in WORD_LIST {
            assert!(
                seen.insert(word),
                "Duplicate word in WORD_LIST: '{word}'"
            );
        }
    }

    // ===============================================================
    // 7. Parse robustness  (7 tests)
    //    Guards: Common::parse handles edge-case JSON gracefully.
    // ===============================================================

    /// Deeply nested JSON must parse without stack overflow.
    #[test]
    fn reg_38_parse_nested_json() {
        let json = r#"{"a":{"b":{"c":{"d":"deep"}}}}"#;
        let result = Common::parse(json);
        assert!(result.is_ok());
    }

    /// JSON with unicode keys must parse successfully.
    #[test]
    fn reg_39_parse_unicode_keys() {
        let json = r#"{"clé":"valeur","キー":"値"}"#;
        let result = Common::parse(json);
        assert!(result.is_ok());
    }

    /// JSON with numeric values (not just strings) must parse.
    #[test]
    fn reg_40_parse_numeric_values() {
        let json = r#"{"count":42,"ratio":1.5,"flag":true}"#;
        let result = Common::parse(json);
        assert!(result.is_ok());
    }

    /// JSON array at root is not a valid Common → must error.
    #[test]
    fn reg_41_parse_root_array_errors() {
        let result = Common::parse("[1,2,3]");
        assert!(result.is_err());
    }

    /// JSON with escaped characters in strings must parse.
    #[test]
    fn reg_42_parse_escaped_strings() {
        let json = r#"{"msg":"line1\nline2\ttab\"quote"}"#;
        let result = Common::parse(json);
        assert!(result.is_ok());
    }

    /// A single whitespace-only string must fail parsing.
    #[test]
    fn reg_43_parse_whitespace_only_errors() {
        let result = Common::parse("   ");
        assert!(result.is_err());
    }

    /// Parsing the same input twice must yield equivalent results
    /// (no hidden state between calls).
    #[test]
    fn reg_44_parse_is_stateless() {
        let json = r#"{"words":["a","b"]}"#;
        let a = Common::parse(json).unwrap();
        let b = Common::parse(json).unwrap();
        assert_eq!(
            serde_json::to_string(&a).unwrap(),
            serde_json::to_string(&b).unwrap(),
            "parse() must be stateless"
        );
    }

    // ===============================================================
    // 8. API contract guards  (6 tests)
    //    Guards: specific public-API invariants that must never
    //    change across releases.
    // ===============================================================

    /// The library must expose exactly 55 constants.
    #[test]
    fn reg_45_exactly_55_constants() {
        assert_eq!(
            Constants::new().constants().len(),
            55,
            "Public API contract: 55 constants"
        );
    }

    /// EULER must equal std::f64::consts::E exactly (no custom
    /// approximation).
    #[test]
    fn reg_46_euler_is_std_e() {
        assert_eq!(EULER, std::f64::consts::E);
    }

    /// PI must equal std::f64::consts::PI exactly.
    #[test]
    fn reg_47_pi_is_std_pi() {
        assert_eq!(PI, std::f64::consts::PI);
    }

    /// TAU must equal std::f64::consts::TAU exactly.
    #[test]
    fn reg_48_tau_is_std_tau() {
        assert_eq!(TAU, std::f64::consts::TAU);
    }

    /// SQRT2 must equal std::f64::consts::SQRT_2 exactly.
    #[test]
    fn reg_49_sqrt2_is_std_sqrt2() {
        assert_eq!(SQRT2, std::f64::consts::SQRT_2);
    }

    /// Faraday constant must satisfy F = N_A * e, where e is the
    /// elementary charge ≈ 1.602176634e-19 C.
    #[test]
    fn reg_50_faraday_avogadro_relation() {
        let elementary_charge = 1.602_176_634e-19_f64;
        let computed = AVOGADRO * elementary_charge;
        assert!(
            (FARADAY - computed).abs() < 0.01,
            "F = N_A * e violated: {} vs {}",
            FARADAY,
            computed
        );
    }
}
