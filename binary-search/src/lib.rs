pub fn find<T: Ord, U: AsRef<[T]>>(array: U, key: T) -> Option<usize> {
    let array_ref = array.as_ref();
    if array_ref.is_empty() {
        return None;
    }

    let mut start = 0;
    let mut mid = array_ref.len() / 2;
    let mut end = array_ref.len();

    while array_ref[mid] != key {
        if key < array_ref[mid] {
            end = mid;
        } else {
            start = mid;
        }
        let new_mid = (start + end) / 2;
        if new_mid == mid {
            return None;
        }
        mid = new_mid;
    }

    return Some(mid);
}
