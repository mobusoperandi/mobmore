use itertools::Itertools;

pub fn build_proverb(list: &[&str]) -> String {
    match list {
        [] => "".to_owned(),
        &[first] => and_all_for(first),
        &[first, ..] => {
            let first_lines = list
                .iter()
                .cloned()
                .tuple_windows()
                .map(for_want)
                .join("\n");
            let last_line = and_all_for(first);

            [first_lines, last_line].join("\n")
        }
    }
}

fn for_want((want, lost): (&str, &str)) -> String {
    format!("For want of a {want} the {lost} was lost.")
}

fn and_all_for(for_a: &str) -> String {
    format!("And all for the want of a {for_a}.")
}
