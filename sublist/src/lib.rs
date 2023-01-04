#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let first_len = _first_list.len();
    let second_len = _second_list.len();
    if first_len == 0 {
        if second_len == 0 {
            return Comparison::Equal;
        }else{
            return Comparison::Sublist;
        }
        return Comparison::Sublist;
    }else if second_len == 0 {
        return Comparison::Superlist;
    }else if first_len == second_len && _first_list == _second_list {
        return Comparison::Equal;
    }

    let mut long_list = _first_list;
    let mut short_list = _second_list;
    let mut first_longer = true;
    if second_len > first_len {
        long_list = _second_list;
        short_list = _first_list;
        first_longer = false;
    }

    let short_len = short_list.len();

    for part in long_list.windows(short_len) {
        if part == short_list {
            if first_longer {
                return Comparison::Superlist;
            }else{
                return Comparison::Sublist;
            }
        }
    }
    return Comparison::Unequal;
}
