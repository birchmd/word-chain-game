pub fn word_chain_game(start: &str, end: &str, words: &[String]) -> Option<usize> {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::word_chain_game;

    #[test]
    fn test_path_exists() {
        let word_list = read_word_list().unwrap();
        let start = "dog";
        let end = "imp";
        assert_eq!(word_chain_game(start, end, &word_list), Some(7));
    }

    #[test]
    fn test_path_does_not_exists() {
        let word_list = read_word_list().unwrap();
        let start = "mad";
        let end = "gnu";
        assert_eq!(word_chain_game(start, end, &word_list), None);
    }

    fn read_word_list() -> Result<Vec<String>, std::io::Error> {
        let words = std::fs::read_to_string("res/three_letter_words.txt")?;
        let word_list = words.split('\n').map(Into::into).collect();
        Ok(word_list)
    }
}
