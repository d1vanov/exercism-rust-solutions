#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(lhs: &[T], rhs: &[T]) -> Comparison {
    if lhs.len() == rhs.len() {
        let mut mismatch_found = false;
        for i in 0..lhs.len() {
            if lhs[i] != rhs[i] {
                mismatch_found = true;
                break;
            }
        }

        if !mismatch_found {
            return Comparison::Equal;
        }
    }

    if left_contains_right(lhs, rhs) {
        return Comparison::Superlist;
    }

    if left_contains_right(rhs, lhs) {
        return Comparison::Sublist;
    }

    return Comparison::Unequal;
}

fn left_contains_right<T: PartialEq>(lhs: &[T], rhs: &[T]) -> bool {
    let lhs_len = lhs.len();
    let rhs_len = rhs.len();
    if rhs_len > lhs_len {
        return false;
    }

    for i in 0..lhs_len-rhs_len+1 {
        let mut mismatch_found = false;
        for j in 0..rhs_len {
            if lhs[i+j] != rhs[j] {
                mismatch_found = true;
                break;
            }
        }

        if mismatch_found {
            continue;
        }

        return true;
    }

    return false;
}
