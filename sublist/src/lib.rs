#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else if is_sublist(_first_list, _second_list) {
        Comparison::Sublist
    } else if is_sublist(_second_list, _first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

pub fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.len() > second_list.len() {
        return false;
    }

    for i in 0..=second_list.len() - first_list.len() {
        if first_list == &second_list[i..i + first_list.len()] {
            return true;
        }
    }

    false
}