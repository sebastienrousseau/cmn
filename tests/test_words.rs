#![allow(missing_docs)]
// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(test)]
mod tests {
    use cmn::{words::WORD_LIST, Words};

    // ---------------------------------------------------------------
    // Words::new
    // ---------------------------------------------------------------

    #[test]
    fn new_creates_empty_set() {
        let words = Words::new();
        assert_eq!(words.count(), 0);
        assert!(words.words_list().is_empty());
    }

    // ---------------------------------------------------------------
    // Words::default — populates from WORD_LIST
    // ---------------------------------------------------------------

    #[test]
    fn default_populates_from_word_list() {
        let words = Words::default();
        assert_eq!(
            words.count(),
            WORD_LIST.len(),
            "Default words count should match WORD_LIST length"
        );
    }

    #[test]
    fn default_words_list_is_sorted() {
        let words = Words::default();
        let list = words.words_list();
        assert_eq!(list[0], "aboard");
        // Verify entire list is sorted
        for window in list.windows(2) {
            assert!(
                window[0] <= window[1],
                "Words list not sorted: '{}' > '{}'",
                window[0],
                window[1]
            );
        }
    }

    // ---------------------------------------------------------------
    // Words::words — returns self reference
    // ---------------------------------------------------------------

    #[test]
    fn words_returns_self_reference() {
        let words = Words::new();
        let ref_words = words.words();
        assert_eq!(ref_words.count(), words.count());
    }

    // ---------------------------------------------------------------
    // Words::add_word / Words::contains
    // ---------------------------------------------------------------

    #[test]
    fn add_word_makes_it_findable() {
        let mut words = Words::new();
        words.add_word("hello");
        assert!(words.contains("hello"));
    }

    #[test]
    fn contains_missing_word_returns_false() {
        let words = Words::new();
        assert!(!words.contains("nonexistent"));
    }

    #[test]
    fn add_duplicate_word_does_not_increase_count() {
        let mut words = Words::new();
        words.add_word("dup");
        words.add_word("dup");
        assert_eq!(words.count(), 1);
    }

    #[test]
    fn add_empty_string_is_stored() {
        let mut words = Words::new();
        words.add_word("");
        assert!(words.contains(""));
        assert_eq!(words.count(), 1);
    }

    // ---------------------------------------------------------------
    // Words::remove_word
    // ---------------------------------------------------------------

    #[test]
    fn remove_existing_word_returns_true() {
        let mut words = Words::new();
        words.add_word("target");
        assert!(words.remove_word("target"));
        assert!(!words.contains("target"));
    }

    #[test]
    fn remove_nonexistent_word_returns_false() {
        let mut words = Words::new();
        assert!(!words.remove_word("ghost"));
    }

    // ---------------------------------------------------------------
    // Words::clear
    // ---------------------------------------------------------------

    #[test]
    fn clear_empties_the_set() {
        let mut words = Words::default();
        assert!(words.count() > 0);
        words.clear();
        assert_eq!(words.count(), 0);
        assert!(words.words_list().is_empty());
    }

    // ---------------------------------------------------------------
    // Words::count
    // ---------------------------------------------------------------

    #[test]
    fn count_reflects_additions_and_removals() {
        let mut words = Words::new();
        assert_eq!(words.count(), 0);
        words.add_word("one");
        assert_eq!(words.count(), 1);
        words.add_word("two");
        assert_eq!(words.count(), 2);
        let _ = words.remove_word("one");
        assert_eq!(words.count(), 1);
    }

    // ---------------------------------------------------------------
    // Words::words_list — sorting behavior
    // ---------------------------------------------------------------

    #[test]
    fn words_list_returns_sorted_vec() {
        let mut words = Words::new();
        words.add_word("zebra");
        words.add_word("apple");
        words.add_word("mango");
        let list = words.words_list();
        assert_eq!(list, vec!["apple", "mango", "zebra"]);
    }

    // ---------------------------------------------------------------
    // Words — derive trait coverage (Clone, Debug, Serialize, Deserialize)
    // ---------------------------------------------------------------

    #[test]
    fn words_clone_is_independent() {
        let mut original = Words::new();
        original.add_word("original");
        let mut cloned = original.clone();
        cloned.add_word("cloned");
        // Original should not have the word added to clone
        assert!(!original.contains("cloned"));
        assert!(cloned.contains("original"));
    }

    #[test]
    fn words_debug_format() {
        let mut words = Words::new();
        words.add_word("test");
        let debug = format!("{:?}", words);
        assert!(debug.contains("Words"));
        assert!(debug.contains("test"));
    }

    #[test]
    fn words_serialize_deserialize_roundtrip() {
        let mut original = Words::new();
        original.add_word("alpha");
        original.add_word("beta");

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Words = serde_json::from_str(&json).unwrap();

        assert_eq!(original.count(), deserialized.count());
        assert!(deserialized.contains("alpha"));
        assert!(deserialized.contains("beta"));
    }

    // ---------------------------------------------------------------
    // WORD_LIST constant
    // ---------------------------------------------------------------

    #[test]
    fn word_list_is_not_empty() {
        assert!(!WORD_LIST.is_empty());
    }

    #[test]
    fn word_list_contains_no_empty_strings() {
        for word in WORD_LIST {
            assert!(
                !word.is_empty(),
                "WORD_LIST should not contain empty strings"
            );
        }
    }

    // ---------------------------------------------------------------
    // FromIterator
    // ---------------------------------------------------------------

    #[test]
    fn from_iterator_creates_words() {
        let items = vec!["alpha".to_string(), "beta".to_string()];
        let words: Words = items.into_iter().collect();
        assert_eq!(words.count(), 2);
        assert!(words.contains("alpha"));
        assert!(words.contains("beta"));
    }

    #[test]
    fn from_iterator_deduplicates() {
        let items = vec!["dup".to_string(), "dup".to_string()];
        let words: Words = items.into_iter().collect();
        assert_eq!(words.count(), 1);
    }
}
