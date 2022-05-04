mod tokenize;

#[cfg(test)]
mod tests {
    #[test]
    fn test_word_tokenize() {
        use crate::tokenize::tokenize;
        let tokenized = tokenize::word_tokenize("a b c");
        assert_eq!(tokenized.len(), 3);
    }
    #[test]
    fn test_word_tokenize_fail() {
        use crate::tokenize::tokenize;
        let tokenized = tokenize::word_tokenize("a b c");

        assert_ne!(tokenized.len(), 4);
    }

    #[test]
    fn test_sent_tokenize(){
        use crate::tokenize::tokenize;
        let tokenized = tokenize::sent_tokenize("Let's go to the mall. I want to buy a shirt. It has to be blue.");
        let len = tokenized.len();

        assert_eq!(len, 3);
        
        let last_item = tokenized[..].last().copied().expect("Unpacking vector error");
        assert_ne!(last_item, "");
    }
}
