struct Bracket {
    kind: Kind,
    side: Side,
}

#[derive(PartialEq, Eq)]
enum Kind {
    Bracket,
    Brace,
    Paren,
}

enum Side {
    Open,
    Close,
}

impl TryFrom<char> for Bracket {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '[' => Ok(Self {
                kind: Kind::Bracket,
                side: Side::Open,
            }),
            ']' => Ok(Self {
                kind: Kind::Bracket,
                side: Side::Close,
            }),
            '{' => Ok(Self {
                kind: Kind::Brace,
                side: Side::Open,
            }),
            '}' => Ok(Self {
                kind: Kind::Brace,
                side: Side::Close,
            }),
            '(' => Ok(Self {
                kind: Kind::Paren,
                side: Side::Open,
            }),
            ')' => Ok(Self {
                kind: Kind::Paren,
                side: Side::Close,
            }),
            _ => Err(()),
        }
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    string
        .chars()
        .try_fold(vec![], |mut stack, character| {
            let Ok(Bracket { kind, side }) = Bracket::try_from(character) else {
                return Some(stack);
            };

            match (side, stack.last()) {
                (Side::Open, _) => {
                    stack.push(kind);
                }
                (Side::Close, Some(last_kind)) if kind == *last_kind => {
                    stack.pop();
                }
                _ => {
                    return None;
                }
            };

            Some(stack)
        })
        .is_some_and(|stack| stack.is_empty())
}
