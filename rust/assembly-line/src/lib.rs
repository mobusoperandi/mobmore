pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate = speed as f64 * 221.0;
    let success_rate = match speed {
        0..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => unreachable!("Congratulations, you broke physics"),
    };

    rate * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
