// Copyright © 2023 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(test)]
mod tests {
    pub use cmn::Words;
    use cmn::*;

    #[test]
    fn test_new() {
        let constants = Constants::new();
        assert!(constants.is_valid());
        assert!(constants.constants.len() >= 9);
        assert!(constants.constants.len() <= 16);
    }

    #[test]
    fn test_common_new() {
        let constants = Constants::new();
        assert!(constants.is_valid());
        assert!(constants.constants.len() >= 9);
    }

    #[test]
    fn test_constants() {
        let constants = Constants::new();
        let new_constants = constants.constants();

        assert_eq!(new_constants.len(), 16);
        assert_eq!(new_constants, constants.constants());
    }

    #[test]
    fn test_words() {
        let common = Common::new();
        let words = common.words();
        assert_eq!(words.words_list().len(), 4096);
    }

    #[test]
    fn test_default() {
        let common = Common::default();
        let constants = common.constants();

        assert_eq!(constants.constants().len(), 16);
        assert_eq!(constants.constants(), Constants::default().constants());
    }

    #[test]
    fn test_constants_method() {
        let constants = Constants::new();
        let new_constants = constants.constants();

        assert_eq!(new_constants.len(), 16);
        assert_eq!(new_constants, Constants::default().constants());
    }

    #[test]
    fn test_constants_method_returns_copy_of_constants() {
        let constants = Constants::new();
        let new_constants = constants.constants().to_vec();

        assert_eq!(new_constants.len(), 16);
        assert_eq!(constants.constants().len(), 16);
        assert_eq!(new_constants, constants.constants().to_vec());
    }

    #[test]
    fn test_constants_method_returns_constants_instance() {
        let constants = Constants::new();
        let new_constants = constants.constants();

        assert_eq!(new_constants.len(), 16);
        assert_eq!(new_constants, Constants::default().constants());
    }

    #[test]
    fn test_constants_method_returns_default_constants_instance() {
        let constants = Constants::new();
        let binding = Constants::default();
        let default_constants = binding.constants();

        assert_eq!(default_constants.len(), 16);
        assert_eq!(default_constants, constants.constants());
    }
}
