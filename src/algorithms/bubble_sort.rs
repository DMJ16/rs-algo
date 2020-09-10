pub fn bubble_sort<T: PartialOrd>(vec: &mut [T]) {
    for j in 0..vec.len() {
        let mut is_sorted = true;
        for i in 0..(vec.len() - 1) - j {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                is_sorted = false;
            }
        }
        if is_sorted {
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v1 = vec![10, 5, 2, 3];
        let mut v2 = vec![11, 7, 1, 14];
        let mut v3 = vec![3, 1, 7, 11];
        let mut v4 = vec![100, 200, 300, 400];
        let mut v5 = vec![-3, 4, 0, -2, 6, -1];
        let mut v6 = vec![1, 4, 2, 10, 23, 3, 1, 0, 20];
        let mut v7 = vec![-3];
        bubble_sort(&mut v1);
        bubble_sort(&mut v2);
        bubble_sort(&mut v3);
        bubble_sort(&mut v4);
        bubble_sort(&mut v5);
        bubble_sort(&mut v6);
        bubble_sort(&mut v7);

        assert_eq!(v1, vec![2, 3, 5, 10]);
        assert_eq!(v2, vec![1, 7, 11, 14]);
        assert_eq!(v3, vec![1, 3, 7, 11]);
        assert_eq!(v4, vec![100, 200, 300, 400]);
        assert_eq!(v5, vec![-3, -2, -1, 0, 4, 6,]);
        assert_eq!(v6, vec![0, 1, 1, 2, 3, 4, 10, 20, 23,]);
        assert_eq!(v7, vec![-3]);
    }
}
