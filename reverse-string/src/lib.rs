use std::str::FromStr;

pub fn reverse(input: &str) -> String {
    // let mut vec = Vec::new();
    // for i in input.chars() {
    //     vec.push(i);
    // }
    // vec.reverse();
    // vec.iter().collect::<String>()
    input.chars().rev().collect()
}