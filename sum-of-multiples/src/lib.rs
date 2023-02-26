//use std::collections::HashMap;
use std::collections::HashSet;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // let mut sum = 0;
    // //let mut hash = HashMap::new();
    // let mut hset = HashSet::new();

    // for i in factors {
    //     if *i > 0 {
    //         let mut count = 1;
    //         let mut multi = i * count;
    //         while multi < limit {
    //             //let acc = hash.entry(multi).or_insert(0);
    //             // if *acc == 0{
    //             //     sum += multi;
    //             // }
    //             // *acc += 1;

    //             let insert_ok = hset.insert(multi);
    //             if insert_ok {
    //                 sum += multi;
    //             }

    //             count += 1;
    //             multi = i * count;
    //         }
    //     }
    // }
    // sum

    (1..limit)
        .filter(|x| factors.iter().any(|m| *m != 0 && x % m == 0))
        .sum()
}
