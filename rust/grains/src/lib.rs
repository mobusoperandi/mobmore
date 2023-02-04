use core::ops::RangeInclusive;

const CELL_RANGE: RangeInclusive<u32> = 1..=64;

pub fn square(s: u32) -> u64 {
    if !CELL_RANGE.contains(&s) {
        panic!("Square must be between 1 and 64");
    }
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    CELL_RANGE.map(square).sum()
}
