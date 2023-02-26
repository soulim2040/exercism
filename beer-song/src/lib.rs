
pub fn verse(n: u32) -> String {
    return  match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        // m @ 3..=99 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", m, m, m -1),
        // _ => "no this bottle".to_string(),
        _ => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n -1),
    };
}

pub fn sing(start: u32, end: u32) -> String {
    // let mut result = String::new();
    // for i in (end..=start).rev() {
    //     if i != start {
    //         result.push_str("\n");
    //     }
    //     result.push_str(&verse(i));
    // }
    // result
    (end .. start+1).rev()
        .map(|x| verse(x))
        .collect::<Vec<_>>()
        .join("\n")
}
