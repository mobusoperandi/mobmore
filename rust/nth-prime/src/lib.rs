pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(is_prime_by_division)
        .nth(n.try_into().unwrap())
        .unwrap()
}

fn is_prime_by_division(n: &u32) -> bool {
    let is_divisible_by = |divisor| *n % divisor == 0;
    !(2..*n).any(is_divisible_by)
}
