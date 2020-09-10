pub fn shell_sort<T: PartialOrd + Copy>(collection: &[T]) -> Vec<T> {
    let len = collection.len();
    let mut mid = len / 2;
    let mut result: Vec<T> = collection.to_vec();

    while mid > 0 {
        for i in mid..len {
            let temp = result[i];
            let mut j = i;
            while j >= mid && result[j - mid] > temp {
                result[j] = result[j - mid];
                j -= mid;
            }
            result[j] = temp;
        }
        mid /= 2;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shell_sort() {
        let mut v1 = vec![10, 5, 2, 3];
        let mut v2 = vec![11, 7, 1, 14];
        let mut v3 = vec![3, 1, 7, 11];
        let mut v4 = vec![100, 200, 300, 400];
        let mut v5 = vec![-3, 4, 0, -2, 6, -1];
        let mut v6 = vec![1, 4, 2, 10, 23, 3, 1, 0, 20];
        let mut v7 = vec![-3];
        v1 = shell_sort(&v1);
        v2 = shell_sort(&v2);
        v3 = shell_sort(&v3);
        v4 = shell_sort(&v4);
        v5 = shell_sort(&v5);
        v6 = shell_sort(&v6);
        v7 = shell_sort(&v7);

        assert_eq!(v1, vec![2, 3, 5, 10]);
        assert_eq!(v2, vec![1, 7, 11, 14]);
        assert_eq!(v3, vec![1, 3, 7, 11]);
        assert_eq!(v4, vec![100, 200, 300, 400]);
        assert_eq!(v5, vec![-3, -2, -1, 0, 4, 6,]);
        assert_eq!(v6, vec![0, 1, 1, 2, 3, 4, 10, 20, 23,]);
        assert_eq!(v7, vec![-3]);
    }
}
