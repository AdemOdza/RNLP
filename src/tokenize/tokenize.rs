
use regex::Regex;

pub fn sent_tokenize(s: &str) -> Vec<&str> {
	let p = Regex::new("[!.?]").expect("Invalid Regex");
	let split_sent: Vec<&str> = p.split(s).into_iter().collect();
	let leng = split_sent.len();

	if split_sent[leng - 1] == ""{
		return split_sent[0..leng-1].to_vec();
	}
	split_sent
}



//Simple Tokenizers
pub fn word_tokenize(s: &str) -> Vec<&str> {
	s.split(" ").collect()
}

pub fn line_tokenize(s: &str) -> Vec<&str> {
	s.split("\n").collect()
}
