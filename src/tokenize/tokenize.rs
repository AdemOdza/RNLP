pub fn word_tokenize(s: &str) -> Vec<&str> {
	s.split(" ").collect()
}

pub fn sent_tokenize(s: &str) -> Vec<&str> {
	s.split(".").collect()
}
