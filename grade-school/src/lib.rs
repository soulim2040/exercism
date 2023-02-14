// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
use std::collections::HashMap;

#[allow(clippy::new_without_default)]
pub struct School {
    roster : HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Self {
            roster : HashMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster.entry(grade).or_insert(Vec::<String>::new()).push(student.into());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut vec : Vec<u32> = self.roster.keys().cloned().collect();
        vec.sort();
        vec
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        if let Some(v) = self.roster.get(&grade){
            let mut new_names : Vec<String> = v.clone();
            new_names.sort();
            new_names
        }else{
            Vec::<String>::new()
        }
    }
}
