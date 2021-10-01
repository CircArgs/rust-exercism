use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match _first_list.len().cmp(&(_second_list.len())) {
        Ordering::Less => helper(_first_list, _second_list, Comparison::Sublist),
        Ordering::Greater => helper(_second_list, _first_list, Comparison::Superlist),
        Ordering::Equal => {
            if _second_list == _first_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }
}

fn helper<T: PartialEq>(_first_list: &[T], _second_list: &[T], ret: Comparison) -> Comparison {
    for a in _second_list.windows(_first_list.len()) {
        if a == &_first_list[..] {
            return ret;
        }
    }

    Comparison::Unequal
}
