#[cfg(test)]
mod tests {
    pub use cmn::Words;
    use cmn::*;

    #[test]
    fn test_new() {
        let new_constant = Constants.constants();
        let constants = new_constant;
        assert!(constants.len() > 0);
    }

    #[test]
    fn test_constants() {
        let constants = Constants.constants();
        assert!(constants.len() >= 9);
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
        assert_eq!(common.constants().constants().len(), 12);
    }
}
