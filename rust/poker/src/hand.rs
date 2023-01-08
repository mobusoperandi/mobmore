use crate::card::{Card, Rank};
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};

use std::str::FromStr;
use std::sync::RwLock;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Hand<'a> {
    origin_str: &'a str,
    cards: [Card; 5],
}

impl<'a> TryFrom<&'a str> for Hand<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            origin_str: value,
            cards: parse_five_cards(value)?,
        })
    }
}

impl<'a> Hand<'a> {
    pub(crate) fn cmp_rank(&self, other: &Self) -> std::cmp::Ordering {
        let category_ordering = self.category().cmp(&other.category());
        if category_ordering != std::cmp::Ordering::Equal {
            return category_ordering;
        }
        self.discriminants().cmp(&other.discriminants())
    }
    pub(crate) fn origin_str(self) -> &'a str {
        self.origin_str
    }

    fn category(&self) -> Category {
        use Category::*;
        if self.has_flush() && self.has_straight() {
            StraightFlush
        } else if self.has_n_of_a_kind(4) {
            FourOfAKind
        } else if self.has_n_of_a_kind(3) && self.has_n_of_a_kind(2) {
            FullHouse
        } else if self.has_flush() {
            Flush
        } else if self.has_straight() {
            Straight
        } else if self.has_n_of_a_kind(3) {
            ThreeOfAKind
        } else if self.has_two_pair() {
            TwoPair
        } else if self.has_n_of_a_kind(2) {
            Pair
        } else {
            HighCard
        }
    }

    pub(crate) fn has_flush(&self) -> bool {
        self.cards.iter().map(|card| card.suit).all_equal()
    }

    pub(crate) fn has_straight(&self) -> bool {
        self.has_low_straight()
            || self
                .ranks_descending()
                .into_iter()
                .map(u8::from)
                .tuple_windows()
                .all(|(a, b)| a == b + 1)
    }

    pub(crate) fn has_n_of_a_kind(&self, n: usize) -> bool {
        self.rank_counts()
            .into_iter()
            .any(|(_rank, count)| count == n)
    }

    pub(crate) fn has_two_pair(&self) -> bool {
        self.rank_counts()
            .into_iter()
            .filter(|(_rank, count)| *count == 2)
            .count()
            == 2
    }

    fn discriminants(&self) -> Vec<Rank> {
        use Category::*;
        match self.category() {
            StraightFlush | Straight => vec![self.straight_discriminant()],
            Flush => self.ranks_descending().into(),
            _ => self.n_of_a_kind_discriminants(),
        }
    }

    fn has_low_straight(&self) -> bool {
        use Rank::*;
        self.ranks_descending() == [Ace, Five, Four, Three, Two]
    }

    fn ranks_descending(&self) -> [Rank; 5] {
        let mut ranks = self.ranks();
        ranks.sort();
        ranks.reverse();
        ranks
    }

    fn ranks(&self) -> [Rank; 5] {
        self.cards.map(|card| card.rank)
    }

    fn rank_counts(&self) -> HashMap<Rank, usize> {
        static CACHE: RwLock<BTreeMap<[Rank; 5], HashMap<Rank, usize>>> =
            RwLock::new(BTreeMap::new());
        let ranks = self.ranks();
        if let Some(counts) = CACHE.read().unwrap().get(&ranks) {
            return counts.clone();
        }
        let counts = ranks.into_iter().counts();
        CACHE.write().unwrap().insert(ranks, counts.clone());
        counts
    }

    pub(crate) fn straight_discriminant(&self) -> Rank {
        if self.has_low_straight() {
            Rank::Five
        } else {
            self.ranks_descending()[0]
        }
    }

    pub(crate) fn n_of_a_kind_discriminants(&self) -> Vec<Rank> {
        self.rank_counts_descending()
            .into_iter()
            .map(|(rank, _count)| rank)
            .collect()
    }

    fn rank_counts_descending(&self) -> Vec<(Rank, usize)> {
        self.rank_counts()
            .into_iter()
            .sorted_by_key(|&(rank, count)| (count, rank))
            .rev()
            .collect()
    }
}

fn parse_five_cards(input: &str) -> Result<[Card; 5], String> {
    let cards = input
        .split(' ')
        .map(Card::from_str)
        .collect::<Result<Vec<Card>, String>>()?
        .try_into()
        .map_err(|cards: Vec<_>| format!("invalid length: {}", cards.len()))?;
    Ok(cards)
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}
