pub fn binary_search<T: PartialOrd>(arr: &[T], targ: &T) -> Result<usize, usize> {
    let mut start = 0;
    let mut end = arr.len() - 1;
    let mut mid = (start + end) / 2;

    while &arr[mid] != targ && start <= end {
        if targ < &arr[mid] {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
        mid = (start + end) / 2;
    }

    if &arr[mid] == targ {
        Ok(mid)
    } else {
        Err(mid + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn binary_search_test() {
        assert_eq!(binary_search(&[1, 2, 58, 99, 100, 500, 2, 1], &100), Ok(4));
        assert_eq!(binary_search(&[1, 2, 5, 99, 10], &5), Ok(2));
        assert_eq!(binary_search(&[2, 5, 6, 9, 13, 15, 28, 30], &13), Ok(4));
        assert_eq!(binary_search(&[2, 5, 6, 9, 13, 15, 28, 30], &1000), Err(8));
        assert_eq!(binary_search(&[2, 5, 6, 9, 13, 15, 28, 30], &50), Err(8));
        assert_eq!(binary_search(&[2, 5, 6, 9, 13, 15, 28, 30], &3), Err(1));
    }
}
