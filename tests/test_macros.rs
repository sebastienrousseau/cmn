#[cfg(test)]
mod tests {

    // Importing cmn crate and all of its macros
    extern crate cmn;
    use cmn::{
        cmn_assert, cmn_contains, cmn_in_range, cmn_join, cmn_max,
        cmn_min, cmn_print, cmn_print_vec, cmn_split, cmn_vec,
        constants::*,
    };

    #[test]
    #[should_panic(expected = "Assertion failed!")]
    fn test_cmn_assert_fail() {
        // Test that cmn_assert! macro correctly triggers a panic when the argument is false
        cmn_assert!(false);
    }

    #[test]
    fn test_cmn_assert() {
        // Test that cmn_assert! macro does not trigger a panic when the argument is true
        cmn_assert!(true);
    }

    #[test]
    fn test_cmn_join() {
        // Test that cmn_join! macro correctly joins the string arguments together
        let s = cmn_join!("Hello", " ", "World");
        assert_eq!(s, "Hello World");
    }

    #[test]
    fn test_cmn_min() {
        // Test that cmn_min! macro correctly identifies the minimum value among the arguments
        assert_eq!(cmn_min!(10, 20, 30), 10);
    }

    #[test]
    fn test_cmn_max() {
        // Test that cmn_max! macro correctly identifies the maximum value among the arguments
        assert_eq!(cmn_max!(10, 20, 30), 30);
    }

    #[test]
    fn test_cmn_print() {
        // Test that cmn_print! macro correctly prints the argument
        cmn_print!("Hello, World!");
    }

    #[test]
    fn test_cmn_print_vec() {
        // Test that cmn_print_vec! macro correctly prints the elements of the vector argument
        cmn_print_vec!(&[1, 2, 3]);
    }

    #[test]
    fn test_cmn_split() {
        // Test that cmn_split! macro correctly splits the string argument into a vector of words
        let v = cmn_split!("Hello World");
        assert_eq!(v, vec!["Hello", "World"]);
    }

    #[test]
    fn test_cmn_vec() {
        // Test that cmn_vec! macro correctly creates a vector from the arguments
        let v = cmn_vec!(1, 2, 3);
        assert_eq!(v, &[1, 2, 3]);
    }

    #[test]
    fn test_cmn_contains() {
        // Test that cmn_contains! macro correctly checks if the first string contains the second
        assert!(cmn_contains!("Hello", "H"));
        assert!(!cmn_contains!("Hello", "x"));
    }

    #[test]
    fn test_cmn_in_range() {
        // Test that cmn_in_range! macro correctly checks if a number is within a range
        assert!(cmn_in_range!(10, 0, 100));
        assert!(!cmn_in_range!(-10, 0, 100));
    }

    #[test]
    fn test_cmn_constants() {
        // Test that each constant defined by the cmn_constants! macro has the expected value or is not empty
        assert_eq!(
            AVOGADRO, 6.02214076e23,
            "AVOGADRO should have a specific value"
        );
        assert_eq!(
            BOLTZMANN, 1.380648e-23,
            "BOLTZMANN should have a specific value"
        );
        assert_eq!(
            EULER, std::f64::consts::E,
            "EULER should have a specific value"
        );
        assert_eq!(
            GAMMA, 0.5772156649015329,
            "GAMMA should have a specific value"
        );
        assert_eq!(
            HASH_ALGORITHM, "Blake3",
            "HASH_ALGORITHM should have a specific value"
        );
        assert_eq!(
            HASH_COST, 8,
            "HASH_COST should have a specific value"
        );
        assert_eq!(
            HASH_LENGTH, 32,
            "HASH_LENGTH should have a specific value"
        );
        assert_eq!(
            PHI,
            (1.0 + SQRT5) / 2.0,
            "PHI should have a specific value"
        );
        assert_eq!(
            PI,
            std::f64::consts::PI,
            "PI should have a specific value"
        );
        assert_eq!(
            PLANCK, 6.62607015e-34,
            "PLANCK should have a specific value"
        );
        assert_eq!(
            SILVER_RATIO,
            1.0 + SQRT2,
            "SILVER_RATIO should have a specific value"
        );
        assert_eq!(
            SPECIAL_CHARS,
            &[
                '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_',
                '+', '=', '[', ']', '{', '}', '|', ';', ':', '"', '<',
                '>', ',', '.', '?', '/', '~', '`'
            ],
            "SPECIAL_CHARS should have a specific value"
        );
        assert_eq!(
            SQRT2,
            std::f64::consts::SQRT_2,
            "SQRT2 should have a specific value"
        );
        assert_eq!(
            SQRT3, 1.7320508075688772,
            "SQRT3 should have a specific value"
        );
        assert_eq!(
            SQRT5, 2.23606797749979,
            "SQRT5 should have a specific value"
        );
        assert_eq!(TAU, 2.0 * PI, "TAU should have a specific value");
    }
}
