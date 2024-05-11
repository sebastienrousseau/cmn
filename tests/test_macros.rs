#[cfg(test)]
mod tests {
    // Importing the cmn crate and all of its macros
    use cmn::Common;
    use cmn::{
        cmn_assert, cmn_contains, cmn_in_range, cmn_join, cmn_map,
        cmn_max, cmn_min, cmn_parse, cmn_print, cmn_print_vec,
        cmn_split, cmn_to_num, cmn_vec, constants::*,
    };

    /// Test that the `cmn_assert!` macro correctly triggers a panic when the argument is false.
    #[test]
    #[should_panic(expected = "Assertion failed!")]
    fn test_cmn_assert_fail() {
        cmn_assert!(false);
    }

    /// Test that the `cmn_assert!` macro does not trigger a panic when the argument is true.
    #[test]
    fn test_cmn_assert() {
        cmn_assert!(true);
    }

    /// Test that the `cmn_contains!` macro correctly checks if the first string contains the second.
    #[test]
    fn test_cmn_contains() {
        assert!(cmn_contains!("Hello", "H"));
        assert!(!cmn_contains!("Hello", "x"));
    }

    /// Test that the `cmn_in_range!` macro correctly checks if the value is within the given range.
    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_cmn_in_range() {
        debug_assert!(cmn_in_range!(5, 0, 10));
        debug_assert!(!cmn_in_range!(15, 0, 10));
    }

    /// Test that the `cmn_join!` macro correctly joins the string arguments together.
    #[test]
    fn test_cmn_join() {
        let s = cmn_join!("Hello", " ", "World");
        assert_eq!(s, "Hello World");
    }

    /// Test that the `cmn_map!` macro correctly creates a HashMap from the given key-value pairs.
    #[test]
    fn test_cmn_map() {
        let map = cmn_map!("foo" => 1, "bar" => 2, "baz" => 3);
        assert_eq!(map.get("foo"), Some(&1));
        assert_eq!(map.get("bar"), Some(&2));
        assert_eq!(map.get("baz"), Some(&3));
        assert_eq!(map.get("qux"), None);
    }

    /// Test that the `cmn_max!` macro correctly identifies the maximum value among the arguments.
    #[test]
    fn test_cmn_max() {
        assert_eq!(cmn_max!(10, 20, 30), 30);
    }

    /// Test that the `cmn_min!` macro correctly identifies the minimum value among the arguments.
    #[test]
    fn test_cmn_min() {
        assert_eq!(cmn_min!(10, 20, 30), 10);
    }

    /// Test that the `cmn_parse!` macro correctly parses the input JSON string into a `Common` instance.
    #[test]
    fn test_cmn_parse() {
        let json = r#"{"words": ["foo", "bar", "baz"]}"#;
        let common = cmn_parse!(json).unwrap();
        let words_list = common.words().words_list();
        assert_eq!(words_list, vec!["bar", "baz", "foo"]);
    }

    /// Test that the `cmn_print!` macro correctly prints the argument.
    #[test]
    fn test_cmn_print() {
        cmn_print!("Hello, World!");
    }

    /// Test that the `cmn_print_vec!` macro correctly prints the elements of the vector argument.
    #[test]
    fn test_cmn_print_vec() {
        cmn_print_vec!(&[1, 2, 3]);
    }

    /// Test that the `cmn_split!` macro correctly splits the string argument into a vector of words.
    #[test]
    fn test_cmn_split() {
        let v = cmn_split!("Hello World");
        assert_eq!(v, vec!["Hello", "World"]);
    }

    /// Test that the `cmn_to_num!` macro correctly converts a string to a number.
    #[test]
    fn test_cmn_to_num() {
        assert_eq!(cmn_to_num!("42.5"), 42.5);
        assert_eq!(cmn_to_num!("invalid"), 0.0);
    }

    /// Test that the `cmn_vec!` macro correctly creates a vector from the arguments.
    #[test]
    fn test_cmn_vec() {
        let v = cmn_vec!(1, 2, 3);
        assert_eq!(v, &[1, 2, 3]);
    }

    /// Test that each constant defined by the `cmn_constants!` macro has the expected value or is not empty.
    #[test]
    fn test_cmn_constants() {
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
            PLANCK_REDUCED,
            PLANCK / (2.0 * PI),
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
        assert_eq!(
            TAU,
            2.0 * std::f64::consts::PI,
            "TAU should have a specific value"
        );
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
