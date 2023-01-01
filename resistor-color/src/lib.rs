use int_enum::IntEnum;
use enum_iterator::{all, Sequence};

use std::fmt;

#[repr(u32)]
#[derive(Debug, Clone, Ord, PartialOrd, Copy, PartialEq, Eq, Sequence, IntEnum)]
pub enum ResistorColor {
    Black       = 0,
    Blue        = 6,
    Brown       = 1,
    Green       = 5,
    Grey        = 8,
    Orange      = 3,
    Red         = 2,
    Violet      = 7,
    White       = 9,
    Yellow      = 4,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResistorColor::Black => write!(f, "Black"),
            ResistorColor::Blue => write!(f, "Blue"),
            ResistorColor::Brown => write!(f, "Brown"),
            ResistorColor::Green => write!(f, "Green"),
            ResistorColor::Grey => write!(f, "Grey"),
            ResistorColor::Orange => write!(f, "Orange"),
            ResistorColor::Red => write!(f, "Red"),
            ResistorColor::Violet => write!(f, "Violet"),
            ResistorColor::White => write!(f, "White"),
            ResistorColor::Yellow => write!(f, "Yellow"),
        }
    }
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    // let vec = colors();
    // let val = vec.iter().find(|&x| color_to_value(*x) == value);
    // if let Some(v) = val {
    //     v.to_string()
    // }else{
    //     "value out of range".into()
    // }
    let color = ResistorColor::from_int(value);
    match color {
        Ok(color) => format!("{:?}", color),
        _  => "value out of range".into()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut vec = all::<ResistorColor>().collect::<Vec<ResistorColor>>();
    vec.sort();
    vec
}
