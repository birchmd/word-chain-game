use std::collections::{HashSet, VecDeque};

pub fn word_chain_game(start: &str, end: &str, words: &[String]) -> Option<usize> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut q: VecDeque<(&str, usize)> = VecDeque::new();

    q.push_back((start, 0));

    loop {
        let (s, l) = q.pop_front()?;

        visited.insert(s);

        if s == end {
            return Some(l);
        }

        let l = l + 1;
        for w in words {
            if !visited.contains(w.as_str()) && hamming_distance(s, w) == 1 {
                q.push_back((w, l));
            }
        }
    }
}

fn hamming_distance(s1: &str, s2: &str) -> usize {
    s1.as_bytes()
        .iter()
        .zip(s2.as_bytes())
        .map(|(c1, c2)| if c1 == c2 { 0 } else { 1 })
        .sum()
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
