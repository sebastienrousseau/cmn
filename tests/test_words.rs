// Copyright © 2023 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(test)]
mod tests {
    use cmn::{words::WORD_LIST, Words};

    #[test]
    fn test_empty_words_list() {
        let words = Words::new();
        assert!(words.words_list().is_empty(), "Words list should be empty when initialized without default.");
    }

    #[test]
    fn test_default_words_list_order() {
        let words = Words::default();
        let words_list = words.words_list();
        // Checking the first three elements to verify correct initialization and ordering
        assert_eq!(
            words_list.len(),
            WORD_LIST.len(),
            "Default words list length should match WORD_LIST length."
        );
        assert_eq!(words_list[0], "aboard", "Check that words list is sorted and starts with the first word.");
        assert_eq!(
            words_list[1], "abode",
            "Check the second word in the sorted list."
        );
        assert_eq!(
            words_list[2], "abort",
            "Check the third word in the sorted list."
        );
    }

    #[test]
    fn test_add_word_and_contains() {
        let mut words = Words::new();
        words.add_word("example");
        assert!(
            words.contains("example"),
            "Word 'example' should be found after adding."
        );
        assert!(
            !words.contains("test"),
            "Word 'test' should not be found if not added."
        );
    }

    #[test]
    fn test_word_count() {
        let mut words = Words::new();
        words.add_word("test");
        words.add_word("check");
        words.add_word("rust");
        assert_eq!(words.count(), 3, "Count should be equal to the number of unique words added.");
    }

    #[test]
    fn test_duplicates_not_added() {
        let mut words = Words::new();
        words.add_word("duplicate");
        words.add_word("duplicate");
        assert_eq!(
            words.count(),
            1,
            "Duplicates should not increase the count."
        );
    }

    #[test]
    fn test_clear_words() {
        let mut words = Words::default();
        assert!(!words.words_list().is_empty(), "Words list should not be empty after initialization with default.");
        words.clear();
        assert!(
            words.words_list().is_empty(),
            "Words list should be empty after calling clear()."
        );
    }

    #[test]
    fn test_remove_word() {
        let mut words = Words::new();
        words.add_word("remove");
        assert!(
            words.contains("remove"),
            "Word 'remove' should be found after adding."
        );
        assert!(
            words.remove_word("remove"),
            "remove_word() should return true if the word was removed."
        );
        assert!(
            !words.contains("remove"),
            "Word 'remove' should not be found after removal."
        );
        assert!(
            !words.remove_word("nonexistent"),
            "remove_word() should return false for a nonexistent word."
        );
    }

    #[test]
    fn test_words_list_order() {
        let mut words = Words::new();
        words.add_word("zebra");
        words.add_word("apple");
        words.add_word("monkey");
        let words_list = words.words_list();
        assert_eq!(
            words_list[0], "apple",
            "Words list should be sorted alphabetically."
        );
        assert_eq!(
            words_list[1], "monkey",
            "Words list should be sorted alphabetically."
        );
        assert_eq!(
            words_list[2], "zebra",
            "Words list should be sorted alphabetically."
        );
    }
}
