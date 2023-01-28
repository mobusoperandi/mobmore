use itertools::intersperse;

fn n_bottles_of_beer_on_the_wall(n: String, pluraliser: &str) -> String {
    format!("{n} bottle{pluraliser} of beer on the wall")
}

pub fn verse(bottles: u32) -> String {
    let FormattedCount {
        count_sentence_case,
        count_lowercase,
        pluraliser,
    } = format_count(bottles);
    let first_half = n_bottles_of_beer_on_the_wall(count_sentence_case, pluraliser);
    let first_line = format!("{first_half}, {count_lowercase} bottle{pluraliser} of beer.");
    let (first_half, bottles) = if bottles > 0 {
        let subject = if bottles == 1 { "it" } else { "one" };
        (
            format!("Take {subject} down and pass it around"),
            bottles - 1,
        )
    } else {
        ("Go to the store and buy some more".to_owned(), 99)
    };
    let FormattedCount {
        count_lowercase,
        pluraliser,
        ..
    } = format_count(bottles);
    let second_half = n_bottles_of_beer_on_the_wall(count_lowercase, pluraliser);
    let second_line = format!("{first_half}, {second_half}.");
    format!("{first_line}\n{second_line}\n")
}

struct FormattedCount {
    count_sentence_case: String,
    count_lowercase: String,
    pluraliser: &'static str,
}

fn format_count(bottles: u32) -> FormattedCount {
    let (count_sentence_case, count_lowercase) = if bottles > 0 {
        (bottles.to_string(), bottles.to_string())
    } else {
        ("No more".to_owned(), "no more".to_owned())
    };
    let pluraliser = if bottles == 1 { "" } else { "s" };
    FormattedCount {
        count_sentence_case,
        count_lowercase,
        pluraliser,
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let verses = (end..=start).rev().map(verse);
    let newline = "\n".to_owned();
    intersperse(verses, newline).collect()
}
