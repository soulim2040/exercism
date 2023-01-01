// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let percent = match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9|10 => 0.77,
        _ => 0.0_f64,
    };

    221.0_f64 * (speed as f64) * percent
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let mut prod_rate = production_rate_per_hour(speed) / 60.0_f64;
    (prod_rate as u32)
}
