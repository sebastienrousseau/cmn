// Copyright © 2023 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(test)]
mod tests {
    extern crate cmn;
    use cmn::constants::Constants;

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
        assert_eq!(constants.len(), 13);

        let names =
            constants.iter().map(|c| c.name).collect::<Vec<_>>();
        assert!(names.contains(&"EULER"));
        assert!(names.contains(&"GAMMA"));
        assert!(names.contains(&"HASH_ALGORITHM"));
        assert!(names.contains(&"HASH_COST"));
        assert!(names.contains(&"HASH_LENGTH"));
        assert!(names.contains(&"PHI"));
        assert!(names.contains(&"PI"));
        assert!(names.contains(&"PLANCK"));
        assert!(names.contains(&"SILVER_RATIO"));
        assert!(names.contains(&"SPECIAL_CHARS"));
        assert!(names.contains(&"SQRT2"));
        assert!(names.contains(&"SQRT5"));
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
}
