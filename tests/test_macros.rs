#[cfg(test)]
mod tests {

    // Importing cmn crate and all of its macros
    use cmn::{
        cmn_assert, cmn_contains, cmn_join, cmn_max, cmn_min,
        cmn_print, cmn_print_vec, cmn_split, cmn_vec, constants::*,
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
    fn test_cmn_constants() {
        // Test that each constant defined by the cmn_constants! macro has the expected value or is not empty
        assert_eq!(
            APERY, 1.2020569031595942,
            "APERY should have a specific value"
        );
        assert_eq!(
            AVOGADRO, 6.02214076e23,
            "AVOGADRO should have a specific value"
        );
        assert_eq!(
            BOLTZMANN, 1.380648e-23,
            "BOLTZMANN should have a specific value"
        );
        assert_eq!(
            CATALAN, 0.915_965_594_177_219,
            "CATALAN should have a specific value"
        );
        assert_eq!(
            COULOMB, 8.9875517923e9,
            "COULOMB should have a specific value"
        );
        assert_eq!(
            EULER,
            std::f64::consts::E,
            "EULER should have a specific value"
        );
        assert_eq!(
            FARADAY, 96485.33212,
            "FARADAY should have a specific value"
        );
        assert_eq!(
            GAMMA, 0.5772156649015329,
            "GAMMA should have a specific value"
        );
        assert_eq!(
            GAS_CONSTANT, 8.314462618,
            "GAS_CONSTANT should have a specific value"
        );
        assert_eq!(
            GLAISHER_KINKELIN, 1.2824271291006226,
            "GLAISHER_KINKELIN should have a specific value"
        );
        assert_eq!(
            GRAVITATIONAL_CONSTANT, 6.67430e-11,
            "GRAVITATIONAL_CONSTANT should have a specific value"
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
            KHINCHIN, 2.6854520010653064,
            "KHINCHIN should have a specific value"
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
            PLANCK_REDUCED, 1.0545718176461565e-34,
            "PLANCK_REDUCED should have a specific value"
        );
        assert_eq!(
            SILVER_RATIO,
            1.0 + SQRT2,
            "SILVER_RATIO should have a specific value"
        );
        assert_eq!(
            SPEED_OF_LIGHT, 299_792_458.0,
            "SPEED_OF_LIGHT should have a specific value"
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
            SQRT5, 2.236_067_977_499_79,
            "SQRT5 should have a specific value"
        );
        assert_eq!(TAU, 2.0 * PI, "TAU should have a specific value");
        assert_eq!(
            VACUUM_PERMEABILITY, 1.25663706212e-6,
            "VACUUM_PERMEABILITY should have a specific value"
        );
        assert_eq!(
            VACUUM_PERMITTIVITY, 8.8541878128e-12,
            "VACUUM_PERMITTIVITY should have a specific value"
        );
    }
}
