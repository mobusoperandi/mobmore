pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|&candidate| {
            factors
                .iter()
                .copied()
                .filter(|&factor| factor > 0)
                .any(|factor| candidate.divisible_by(factor))
        })
        .sum()
}

trait DivisibleBy {
    fn divisible_by(self, divisor: Self) -> bool;
}

impl<T> DivisibleBy for T
where
    T: std::ops::Rem<T, Output = T> + PartialEq + Default,
{
    fn divisible_by(self, divisor: Self) -> bool {
        let zero = T::default();
        self % divisor == zero
    }
}
