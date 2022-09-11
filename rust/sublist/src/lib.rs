
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    if first == second {
        Comparison::Equal
    } else if is_sublist_of(first, second) {
        Comparison::Sublist
    } else if is_sublist_of(second, first) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn is_sublist_of<T: PartialEq>(candidate: &[T], other: &[T]) -> bool {
    if candidate.is_empty() {
        return true;
    }
    other
        .windows(candidate.len())
        .any(|window_into_other| window_into_other == candidate)
}
