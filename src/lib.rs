mod tokenize;

#[cfg(test)]
mod tests {
    #[test]
    fn test_word_tokenize() {
        use crate::tokenize;
        let tokenized = tokenize::word_tokenize::tokenize("a b c");
        assert_eq!(tokenized.len(), 3);
    }
    #[test]
    fn test_word_tokenize_fail() {
        use crate::tokenize;
        let tokenized = tokenize::word_tokenize::tokenize("a b c");
        assert_ne!(tokenized.len(), 4);
    }
}
