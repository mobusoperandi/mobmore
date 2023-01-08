use std::cmp::Ordering;

pub(crate) fn winners_by<T: Copy>(
    collection: Vec<T>,
    f: impl Fn(&T, &T) -> std::cmp::Ordering,
) -> Vec<T> {
    collection
        .iter()
        .filter(|item| {
            collection
                .iter()
                .all(|other| f(item, other) != Ordering::Less)
        })
        .copied()
        .collect::<Vec<T>>()
}
