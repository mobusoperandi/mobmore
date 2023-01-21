pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let sum: u64 = num_string
        .chars()
        .map(|digit| {
            let digit = digit.to_string();
            let digit: u64 = digit.parse().unwrap();
            digit.pow(num_string.len() as u32)
        })
        .sum();
    sum == num.into()
}
