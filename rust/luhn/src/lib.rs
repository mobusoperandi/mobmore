/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|character| *character != ' ')
        .rev()
        .map(|character| character.to_digit(10))
        .enumerate()
        .map(|(index, digit)| -> Option<(usize, u32)> {
            digit.map(|mut digit| {
                let should_double = index % 2 == 1;
                if should_double {
                    digit *= 2;
                    if digit > 9 {
                        digit -= 9;
                    }
                }
                (index, digit)
            })
        })
        .try_fold((0, 0), |(_index, sum), digit| {
            digit.map(|(index, digit)| (index, sum + digit))
        }) 
        .map(|(index, sum)| {
            let count = index + 1;
            count > 1 && sum % 10 == 0
        })
        .unwrap_or(false)
}
