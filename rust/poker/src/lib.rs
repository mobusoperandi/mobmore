mod card;
mod hand;
mod winners_by;

use hand::Hand;
use std::collections::HashSet;
use winners_by::winners_by;

pub(crate) fn hs_from<'a>(input: &[&'a str]) -> HashSet<&'a str> {
    let mut hs = HashSet::new();
    for item in input.iter() {
        hs.insert(*item);
    }
    hs
}

/// Test that the expected output is produced from the given input
/// using the `winning_hands` function.
///
/// Note that the output can be in any order. Here, we use a HashSet to
/// abstract away the order of outputs.
pub fn test(input: &[&str], expected: &[&str]) {
    assert_eq!(hs_from(&winning_hands(input)), hs_from(expected))
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let hands = hands
        .iter()
        .copied()
        .map(|hand| Hand::try_from(hand).unwrap())
        .collect();
    winners_by(hands, Hand::cmp_rank)
        .into_iter()
        .map(Hand::origin_str)
        .collect()
}
