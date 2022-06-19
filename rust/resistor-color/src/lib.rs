use enum_iterator::IntoEnumIterator;
use int_enum::{IntEnum, IntEnumError};

#[derive(Copy, Clone, Debug, PartialEq, IntEnum, IntoEnumIterator, Eq, PartialOrd, Ord)]
#[repr(usize)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    let result: Result<ResistorColor, IntEnumError<ResistorColor>> = ResistorColor::from_int(value);
    match result {
        Ok(value) => format!("{:?}", value),
        Err(_) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut colors: Vec<ResistorColor> = ResistorColor::into_enum_iter().collect();
    colors.sort();
    colors
}
