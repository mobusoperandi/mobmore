use std::str::FromStr;

use num_enum::IntoPrimitive;
#[derive(Clone, Copy, Debug)]
pub(crate) struct Card {
    pub(crate) suit: Suit,
    pub(crate) rank: Rank,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rank, suit) = match *s.as_bytes() {
            [rank, suit] => (
                Rank::try_from(rank).map_err(|error| format!("{error:?}"))?,
                Suit::try_from(suit).map_err(|error| format!("{error:?}"))?,
            ),
            [one, zero, suit] if one == b'1' && zero == b'0' => (
                Rank::Ten,
                Suit::try_from(suit).map_err(|error| format!("{error:?}"))?,
            ),
            _ => return Err(s.to_string()),
        };
        Ok(Self { suit, rank })
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

impl TryFrom<u8> for Suit {
    type Error = u8;

    fn try_from(suit: u8) -> Result<Self, Self::Error> {
        use Suit::*;
        match suit {
            b'H' => Ok(Hearts),
            b'S' => Ok(Spades),
            b'D' => Ok(Diamonds),
            b'C' => Ok(Clubs),
            byte => Err(byte),
        }
    }
}

#[derive(IntoPrimitive, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
pub(crate) enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<u8> for Rank {
    type Error = u8;

    fn try_from(input: u8) -> Result<Self, Self::Error> {
        use Rank::*;
        match input {
            b'2' => Ok(Two),
            b'3' => Ok(Three),
            b'4' => Ok(Four),
            b'5' => Ok(Five),
            b'6' => Ok(Six),
            b'7' => Ok(Seven),
            b'8' => Ok(Eight),
            b'9' => Ok(Nine),
            b'J' => Ok(Jack),
            b'Q' => Ok(Queen),
            b'K' => Ok(King),
            b'A' => Ok(Ace),
            byte => Err(byte),
        }
    }
}
