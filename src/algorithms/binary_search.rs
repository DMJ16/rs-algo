pub fn binary_search(arr: &[isize], targ: isize) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len() - 1;
    let mut mid = (start + end) / 2;

    while arr[mid] != targ && start <= end {
        if targ < arr[mid] {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
        mid = (start + end) / 2;
    }

    match arr[mid] == targ {
        true => Some(mid),
        false => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn binary_search_test() {
        assert_eq!(binary_search(&[1, 2, 58, 99, 100, 500, 2, 1], 100), Some(4));
        assert_eq!(binary_search(&[1, 2, 5, 99, 10], 5), Some(2));
        assert_eq!(binary_search(&[2, 5, 6, 9, 13, 15, 28, 30], 13), Some(4));
        assert_eq!(binary_search(&[2, 5, 6, 9, 13, 15, 28, 30], 1000), None);
        assert_eq!(binary_search(&[2, 5, 6, 9, 13, 15, 28, 30], 50), None);
    }
}
