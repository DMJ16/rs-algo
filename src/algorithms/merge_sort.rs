pub fn merge_sort<T: PartialOrd>(mut vec: Vec<T>) -> Vec<T> {
    if vec.len() <= 1 {
        return vec;
    }
    let mut result = Vec::with_capacity(vec.len());
    let right = vec.split_off(vec.len() / 2);
    let left = merge_sort(vec);
    let right = merge_sort(right);

    let mut left_iter = left.into_iter();
    let mut right_iter = right.into_iter();
    let mut left_next = left_iter.next();
    let mut right_next = right_iter.next();

    loop {
        match left_next {
            Some(ref left_val) => match right_next {
                Some(ref right_val) => {
                    if right_val < left_val {
                        if let Some(val) = right_next.take() {
                            result.push(val);
                        }
                        right_next = right_iter.next();
                    } else {
                        if let Some(val) = left_next.take() {
                            result.push(val);
                        }
                        left_next = left_iter.next();
                    }
                }
                None => {
                    if let Some(val) = left_next.take() {
                        result.push(val);
                    }
                    result.extend(left_iter);
                    return result;
                }
            },
            None => {
                if let Some(right_val) = right_next {
                    result.push(right_val);
                }
                result.extend(right_iter);
                return result;
            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut v1 = vec![10, 5, 2, 3];
        let mut v2 = vec![11, 7, 1, 14];
        let mut v3 = vec![3, 1, 7, 11];
        let mut v4 = vec![100, 200, 300, 400];
        let mut v5 = vec![-3, 4, 0, -2, 6, -1];
        let mut v6 = vec![1, 4, 2, 10, 23, 3, 1, 0, 20];
        let mut v7 = vec![-3];
        v1 = merge_sort(v1);
        v2 = merge_sort(v2);
        v3 = merge_sort(v3);
        v4 = merge_sort(v4);
        v5 = merge_sort(v5);
        v6 = merge_sort(v6);
        v7 = merge_sort(v7);

        assert_eq!(v1, vec![2, 3, 5, 10]);
        assert_eq!(v2, vec![1, 7, 11, 14]);
        assert_eq!(v3, vec![1, 3, 7, 11]);
        assert_eq!(v4, vec![100, 200, 300, 400]);
        assert_eq!(v5, vec![-3, -2, -1, 0, 4, 6,]);
        assert_eq!(v6, vec![0, 1, 1, 2, 3, 4, 10, 20, 23,]);
        assert_eq!(v7, vec![-3]);
    }
}
