pub fn is_armstrong_number(num: u32) -> bool {
    // let mut vec = Vec::new();
    // let mut i: u64= num as u64;
    // while i >= 10 {
    //     let remainder = i % 10;
    //     i = i / 10;
    //     vec.push(remainder);
    // }

    // vec.push(i % 10);

    // let len = vec.len() as u32;
    // let mut result : u64 = 0;
    // for i in vec.iter() {
    //     result += i.pow(len);
    // }

    // let right = num as u64;
    // return result == right;

    let s = num.to_string();
    let len = s.len() as u32;

    let sum : u64 = s.chars()
            .map(|n| n.to_digit(10).unwrap() as u64)
            .map(|n| n.pow(len))
            .sum();
    sum == (num as u64)
}
