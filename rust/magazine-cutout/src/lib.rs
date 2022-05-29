use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine = WordPile::from_str_slice(magazine);
    let note = WordPile::from_str_slice(note);
    magazine.contains(&note)
}

struct WordPile<'a> {
    hash_map: HashMap<&'a str, u32>,
}

impl<'a> WordPile<'a> {
    fn from_str_slice(words: &'a [&str]) -> Self {
        Self {
            hash_map: words.iter().fold(HashMap::new(), |mut acc, word| {
                let word_count = acc.entry(word).or_default();

                *word_count += 1;

                acc
            }),
        }
    }

    fn contains(&self, other: &Self) -> bool {
        other.hash_map
            .iter()
            .all(|(word, count)| self.hash_map.get(word).copied().unwrap_or_default() >= *count)
    }
}
