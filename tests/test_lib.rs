// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(test)]
mod tests {
    use cmn::*;

    // ---------------------------------------------------------------
    // Common::new / Common::default
    // ---------------------------------------------------------------

    #[test]
    fn new_returns_instance_with_null_fields() {
        let common = Common::new();
        // constants() delegates to Constants::default, so it should work
        let constants = common.constants();
        assert_eq!(constants.constants().len(), 55);
    }

    #[test]
    fn default_delegates_to_new() {
        let a = Common::new();
        let b = Common::default();
        // Both should produce equivalent Constants
        assert_eq!(a.constants().constants().len(), b.constants().constants().len());
    }

    // ---------------------------------------------------------------
    // Common::constants
    // ---------------------------------------------------------------

    #[test]
    fn constants_returns_default_constants_instance() {
        let common = Common::default();
        let constants = common.constants();
        assert_eq!(constants.constants().len(), 55);
        assert_eq!(constants.constants(), Constants::default().constants());
    }

    // ---------------------------------------------------------------
    // Common::words — all code paths
    // ---------------------------------------------------------------

    #[test]
    fn words_null_fields_returns_empty_words() {
        // Common::new() sets fields to Value::Null
        let common = Common::new();
        let words = common.words();
        assert!(words.words_list().is_empty());
    }

    #[test]
    fn words_no_words_key_returns_empty_words() {
        let common = Common::parse(r#"{"other": "value"}"#).unwrap();
        let words = common.words();
        assert!(words.words_list().is_empty());
    }

    #[test]
    fn words_with_populated_array_returns_words() {
        let json = r#"{"words": ["delta", "alpha", "charlie", "bravo"]}"#;
        let common = Common::parse(json).unwrap();
        let words = common.words();

        // words_list returns sorted
        let list = words.words_list();
        assert_eq!(list.len(), 4);
        assert_eq!(list, vec!["alpha", "bravo", "charlie", "delta"]);
    }

    #[test]
    fn words_with_empty_array_returns_empty_words() {
        let json = r#"{"words": []}"#;
        let common = Common::parse(json).unwrap();
        let words = common.words();
        assert!(words.words_list().is_empty());
    }

    // ---------------------------------------------------------------
    // Common::parse — happy paths
    // ---------------------------------------------------------------

    #[test]
    fn parse_valid_json_with_fields_returns_ok() {
        let input = r#"{"field1": "value1", "field2": "value2"}"#;
        let result = Common::parse(input);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_empty_object_returns_default() {
        let result = Common::parse("{}").unwrap();
        // Empty object should return default (null fields)
        let words = result.words();
        assert!(words.words_list().is_empty());
    }

    /// `null` is not a valid JSON object for Common — triggers Data error category.
    #[test]
    fn parse_null_literal_returns_data_error() {
        let result = Common::parse("null");
        assert!(result.is_err());
    }

    // ---------------------------------------------------------------
    // Common::parse — error branches (Syntax, Eof, Data)
    // ---------------------------------------------------------------

    #[test]
    fn parse_syntax_error_returns_err() {
        // Malformed JSON triggers Syntax category
        let result = Common::parse("{invalid json}");
        assert!(result.is_err());
    }

    #[test]
    fn parse_eof_error_returns_err() {
        // Truncated JSON triggers Eof category
        let result = Common::parse("{\"key\":");
        assert!(result.is_err());
    }

    #[test]
    fn parse_completely_invalid_returns_err() {
        let result = Common::parse("not json at all");
        assert!(result.is_err());
    }

    #[test]
    fn parse_empty_string_returns_err() {
        let result = Common::parse("");
        assert!(result.is_err());
    }

    // ---------------------------------------------------------------
    // Common — derive trait coverage (Clone, Debug, Serialize)
    // ---------------------------------------------------------------

    #[test]
    fn common_clone_produces_equivalent_instance() {
        let original = Common::parse(r#"{"words": ["test"]}"#).unwrap();
        let cloned = original.clone();
        let orig_words = original.words().words_list();
        let cloned_words = cloned.words().words_list();
        assert_eq!(orig_words, cloned_words);
    }

    #[test]
    fn common_debug_is_not_empty() {
        let common = Common::new();
        let debug = format!("{:?}", common);
        assert!(!debug.is_empty());
        assert!(debug.contains("Common"));
    }

    #[test]
    fn common_serialize_roundtrip() {
        let original = Common::parse(r#"{"key": "value"}"#).unwrap();
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Common = serde_json::from_str(&json).unwrap();
        let re_json = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(json, re_json);
    }

    // ---------------------------------------------------------------
    // run() — direct invocation
    // ---------------------------------------------------------------

    #[test]
    fn run_success_returns_ok() {
        // Ensure CMN_TEST_MODE is not set
        std::env::remove_var("CMN_TEST_MODE");
        let result = run();
        assert!(result.is_ok());
    }

    #[test]
    fn run_test_mode_returns_err() {
        std::env::set_var("CMN_TEST_MODE", "1");
        let result = run();
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert_eq!(err_msg, "Simulated error");
        // Clean up
        std::env::remove_var("CMN_TEST_MODE");
    }

    #[test]
    fn run_test_mode_unset_returns_ok() {
        std::env::remove_var("CMN_TEST_MODE");
        let result = run();
        assert!(result.is_ok());
    }

    #[test]
    fn run_test_mode_zero_returns_ok() {
        std::env::set_var("CMN_TEST_MODE", "0");
        let result = run();
        assert!(result.is_ok());
        std::env::remove_var("CMN_TEST_MODE");
    }
}
