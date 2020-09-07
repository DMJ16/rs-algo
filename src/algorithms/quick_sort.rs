pub fn quick_sort(vec: &mut [isize]) -> Vec<isize> {
    let len = vec.len();
    if len > 1 {
        let mut j = 0;
        for i in 0..len {
            if vec[i] < vec[len - 1] {
                vec.swap(i, j);
                j += 1;
            }
        }
        vec.swap(j, len - 1);
        quick_sort(&mut vec[0..j]);
        quick_sort(&mut vec[j + 1..len]);
    }
    vec.to_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quick_sort_test() {
        assert_eq!(quick_sort(&mut [10, 5, 2, 3]), vec![2, 3, 5, 10]);
        assert_eq!(quick_sort(&mut [11, 7, 1, 14]), vec![1, 7, 11, 14]);
        assert_eq!(quick_sort(&mut [3, 1, 7, 11]), vec![1, 3, 7, 11]);
        assert_eq!(
            quick_sort(&mut [100, 200, 300, 400]),
            vec![100, 200, 300, 400]
        );
        assert_eq!(
            quick_sort(&mut [-3, 4, 0, -2, 6, -1]),
            vec![-3, -2, -1, 0, 4, 6,]
        );
        assert_eq!(
            quick_sort(&mut [1, 4, 2, 10, 23, 3, 1, 0, 20]),
            vec![0, 1, 1, 2, 3, 4, 10, 20, 23,]
        );
        assert_eq!(quick_sort(&mut [-3]), vec![-3]);
    }
}
