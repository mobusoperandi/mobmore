pub fn abbreviate(phrase: &str) -> String {
    phrase
        .chars()
        .filter(|character| character != &'\'')
        .with_prev()
        .filter(|(prev, current)| match (prev, current) {
            (_prev, character) if !character.is_ascii_alphabetic() => false,
            (None, _character) => true,
            (Some(prev), character) => {
                !prev.is_ascii_alphabetic()
                    || character.is_ascii_uppercase() && prev.is_ascii_lowercase()
            }
        })
        .map(|(_, character)| character.to_ascii_uppercase())
        .collect()
}

struct WithPrev<I, T>
where
    I: Iterator<Item = T>,
{
    iter: I,
    prev: Option<T>,
}

impl<I, T> Iterator for WithPrev<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    type Item = (Option<T>, T);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        let prev = std::mem::replace(&mut self.prev, Some(next.clone()));
        Some((prev, next))
    }
}

trait IteratorWithPrev: Iterator {
    fn with_prev<T>(self) -> WithPrev<Self, T>
    where
        Self: Sized + Iterator<Item = T>,
        T: Clone,
    {
        WithPrev {
            iter: self,
            prev: None,
        }
    }
}

impl<T> IteratorWithPrev for T where T: Iterator {}
