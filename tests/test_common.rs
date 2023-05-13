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
        assert_eq!(constants.constants.len(), 16);
    }

    #[test]
    fn test_words() {
        let common = Common::new();
        let words = common.words();
        assert_eq!(words.words_list().len(), 4096);
    }

    #[test]
    fn test_default() {
        let constants = Constants::default();
        assert_eq!(constants.constants.len(), 16);
    }
}
