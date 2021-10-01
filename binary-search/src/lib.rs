use std::cmp::Ordering;

pub fn find<T: Ord, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
    helper(array.as_ref(), key, 0)
}

fn helper<T: Ord>(array: &[T], key: T, ind: usize) -> Option<usize> {
    match array.len() {
        0 => None,
        1 => {
            if array[0] == key {
                Some(ind)
            } else {
                None
            }
        }
        _ => {
            let middle = array.len() / 2;
            match array[middle].cmp(&key) {
                Ordering::Equal => Some(middle + ind),
                Ordering::Greater => helper(&array[0..middle], key, ind),
                _ => helper(&array[middle + 1..], key, middle + ind + 1),
            }
        }
    }
}
