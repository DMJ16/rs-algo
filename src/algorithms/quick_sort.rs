pub fn quick_sort<T: PartialOrd>(vec: &mut [T]) {
    if vec.len() <= 1 {
        return;
    }
    let pivot_idx = pivot(vec);
    let (left, right) = vec.split_at_mut(pivot_idx);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn pivot<T: PartialOrd>(vec: &mut [T]) -> usize {
    let mut pivot_idx = 0;
    for i in 1..vec.len() {
        if vec[i] < vec[pivot_idx] {
            vec.swap(i, pivot_idx + 1);
            vec.swap(pivot_idx, pivot_idx + 1);
            pivot_idx += 1;
        }
    }
    pivot_idx
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut v1 = vec![10, 5, 2, 3];
        let mut v2 = vec![11, 7, 1, 14];
        let mut v3 = vec![3, 1, 7, 11];
        let mut v4 = vec![100, 200, 300, 400];
        let mut v5 = vec![-3, 4, 0, -2, 6, -1];
        let mut v6 = vec![1, 4, 2, 10, 23, 3, 1, 0, 20];
        let mut v7 = vec![-3];
        quick_sort(&mut v1);
        quick_sort(&mut v2);
        quick_sort(&mut v3);
        quick_sort(&mut v4);
        quick_sort(&mut v5);
        quick_sort(&mut v6);
        quick_sort(&mut v7);

        assert_eq!(v1, vec![2, 3, 5, 10]);
        assert_eq!(v2, vec![1, 7, 11, 14]);
        assert_eq!(v3, vec![1, 3, 7, 11]);
        assert_eq!(v4, vec![100, 200, 300, 400]);
        assert_eq!(v5, vec![-3, -2, -1, 0, 4, 6,]);
        assert_eq!(v6, vec![0, 1, 1, 2, 3, 4, 10, 20, 23,]);
        assert_eq!(v7, vec![-3]);
    }
}
