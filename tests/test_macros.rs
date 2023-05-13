#[cfg(test)]
mod tests {

    extern crate cmn;
    use cmn::*;

    #[test]
    #[should_panic(expected = "Assertion failed!")]
    fn test_cmn_assert_fail() {
        // This should panic with "Assertion failed!"
        cmn_assert!(false);
    }

    #[test]
    fn test_cmn_assert() {
        // This should not panic
        cmn_assert!(true);
    }

    #[test]
    fn test_cmn_join() {
        let s = cmn_join!("Hello", " ", "World");
        assert_eq!(s, "Hello World");
    }

    #[test]
    fn test_cmn_min() {
        assert_eq!(cmn_min!(10, 20, 30), 10);
    }

    #[test]
    fn test_cmn_max() {
        assert_eq!(cmn_max!(10, 20, 30), 30);
    }

    #[test]
    fn test_cmn_print() {
        cmn_print!("Hello, World!");
    }

    #[test]
    fn test_cmn_print_vec() {
        cmn_print_vec!(&[1, 2, 3]);
    }

    #[test]
    fn test_cmn_split() {
        let v = cmn_split!("Hello World");
        assert_eq!(v, vec!["Hello", "World"]);
    }

    #[test]
    fn test_cmn_vec() {
        let v = cmn_vec!(1, 2, 3);
        assert_eq!(v, &[1, 2, 3]);
    }

    #[test]
    fn test_cmn_contains() {
        assert!(cmn_contains!("Hello", "H"));
        assert!(!cmn_contains!("Hello", "x"));
    }

    #[test]
    fn test_cmn_in_range() {
        assert!(cmn_in_range!(10, 0, 100));
        assert!(!cmn_in_range!(-10, 0, 100));
    }
}
