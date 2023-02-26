pub fn build_proverb(list: &[&str]) -> String {
    // let mut result = String::new();

    // for i in 0..list.len() {
    //     if i == list.len() -1 {
    //         result.push_str(format!("And all for the want of a {}.", list[0]).as_str());
    //     }else{
    //         result.push_str(format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]).as_str());
    //     }
    // }
    
    // result

    match list.len() {
        0 => String::new(),
        _ => list.windows(2)
            .map(|x| format!("For want of a {} the {} was lost.", x[0], x[1]))
            .chain(std::iter::once(format!("And all for the want of a {}.", list[0])))
            .collect::<Vec<_>>()
            .join("\n"),
    }
}
