use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let _magazine_word_count: HashMap<&str, u32> =
        magazine.iter().fold(HashMap::new(), |mut acc, &word| {
            let entry = acc.entry(word);

            let count = entry.or_default();
            *count += 1;

            acc
        });

    let _note_word_count: HashMap<&str, u32> =
        note.iter().fold(HashMap::new(), |mut acc, &word| {
            let entry = acc.entry(word);

            let count = entry.or_default();
            *count += 1;

            acc
        });

    _note_word_count.into_iter().all(|(_a_word, _a_count)| {
        if _magazine_word_count.get(_a_word).copied().unwrap_or_default() >= _a_count {
            return true;
        }

        false
    })
}
