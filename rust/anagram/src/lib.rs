use std::collections::HashSet;
use itertools::Itertools;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_lower_sorted: String = word_lower.chars().sorted_unstable().collect();
    possible_anagrams
        .iter()
        .copied()
        .filter(|&possible_anagram| {
            let possible_anagram_lower = possible_anagram.to_lowercase();
            if possible_anagram_lower == word_lower {
                return false;
            }
            let possible_anagram_sorted: String = possible_anagram_lower.chars().sorted_unstable().collect();
            possible_anagram_sorted == word_lower_sorted
        })
        .collect()
}
