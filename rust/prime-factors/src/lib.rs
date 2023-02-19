pub fn factors(n: u64) -> Vec<u64> {
    let (_quotient, factors) =
        (2..=n)
            .filter(is_prime)
            .fold((n, vec![]), |(n, mut factors), prime| {
                let (divisions, quotient) = get_divisions_and_quotient(n, prime, 0);
                factors.extend(vec![prime; divisions as usize]);
                (quotient, factors)
            });
    factors
}

fn get_divisions_and_quotient(n: u64, prime: u64, divisions: u8) -> (u8, u64) {
    if let Some(quotient) = div_evenly(n, prime) {
        return get_divisions_and_quotient(quotient, prime, divisions + 1);
    }
    (divisions, n)
}

fn is_prime(&n: &u64) -> bool {
    (2..n).all(|divisor| !is_divisible_by(n, divisor))
}

fn is_divisible_by(n: u64, divisor: u64) -> bool {
    n % divisor == 0
}

fn div_evenly(dividend: u64, divisor: u64) -> Option<u64> {
    if is_divisible_by(dividend, divisor) {
        return Some(dividend / divisor);
    }
    None
}
