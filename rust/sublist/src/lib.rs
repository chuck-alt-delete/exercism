#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist(small: &[i32], large: &[i32]) -> bool {
    if small.is_empty() {
        return true;
    }
    if small.len() > large.len() {
        return false;
    }

    for i in 0..=(large.len() - small.len()) {
        if &large[i..i + small.len()] == small {
            return true;
        }
    }
    false
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else if is_sublist(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
