// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    for key in magazine {
        *map1.entry(key).or_insert(0) += 1;
    }

    for key in note {
        *map2.entry(key).or_insert(0) += 1;
    }

    for (key, value) in map2.iter() {
        let val = map1.get(key);
        let ret = match val {
            None => false,
            Some(v) if v >= value => true,
            Some(v) => false, 
        };
        if ret == false{
            return false;
        }
    }

    return true;
}
