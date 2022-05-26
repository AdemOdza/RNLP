
use regex::Regex;

pub fn sent_tokenize(s: String) -> Vec<String> {
	let p = Regex::new("[!.?]").expect("Invalid Regex");
	let split_sent: Vec<&str> = p.split(s.as_str()).collect();
	let leng = split_sent.len();

	if split_sent[leng - 1] == ""{
		return split_sent[0..leng-1].to_vec().into_iter().map(|s| s.to_string()).collect();
	}
	split_sent.into_iter().map(|s| s.to_string()).collect()
}



//Simple Tokenizers
pub fn word_tokenize(s: String) -> Vec<String> {
	s.replace(".", " PERIOD ")
	.replace("!", " EXCLAMATION ")
	.replace("?", " QUESTION ")
	.replace("  ", " ").split(" ").map(|s| s.to_string()).collect::<Vec<String>>()

}

pub fn line_tokenize(s: String) -> Vec<String> {
	s.split("\n").into_iter().map(|s| s.to_string()).collect()
}
