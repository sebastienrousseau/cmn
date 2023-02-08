#[cfg(test)]
mod tests {
    pub use cmn::Words;
    use cmn::*;

    #[test]
    fn test_new() {
        let new_constant = Constants::new();
        let constants = new_constant;
        assert!(constants.is_valid());
        assert!(constants.constants().len() >= 9);
        assert!(constants.constants().len() <= 16);
    }

    #[test]
    fn test_common_new() {
        let common = Common::new();
        assert!(common.constants().is_valid());
        assert!(common.constants().constants().len() >= 9);
    }

    #[test]
    fn test_constants() {
        let constants = Constants::new();
        assert!(constants.constants.len() >= 9);
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
        assert_eq!(common.constants().constants().len(), 13);
    }
}
