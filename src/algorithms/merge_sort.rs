pub fn merge_sort(vec: &mut [isize]) -> Vec<isize> {
    if vec.len() <= 1 {
        return vec.to_vec();
    }
    let mid = vec.len() / 2;
    let left = merge_sort(&mut vec[..mid]);
    let right = merge_sort(&mut vec[mid..]);
    merge(&left, &right)
}

fn merge(vec1: &[isize], vec2: &[isize]) -> Vec<isize> {
    let len1 = vec1.len();
    let len2 = vec2.len();
    let mut i = 0;
    let mut j = 0;
    let mut result: Vec<isize> = vec![];

    while i < len1 && j < len2 {
        if vec1[i] <= vec2[j] {
            result.push(vec1[i]);
            i += 1;
        } else {
            result.push(vec2[j]);
            j += 1;
        }
    }

    while i < len1 {
        result.push(vec1[i]);
        i += 1;
    }

    while j < len2 {
        result.push(vec2[j]);
        j += 1;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn merge_sort_test() {
        assert_eq!(merge_sort(&mut [10, 5, 2, 3]), vec![2, 3, 5, 10]);
        assert_eq!(merge_sort(&mut [11, 7, 1, 14]), vec![1, 7, 11, 14]);
        assert_eq!(merge_sort(&mut [3, 1, 7, 11]), vec![1, 3, 7, 11]);
        assert_eq!(
            merge_sort(&mut [100, 200, 300, 400]),
            vec![100, 200, 300, 400]
        );
        assert_eq!(
            merge_sort(&mut [-3, 4, 0, -2, 6, -1]),
            vec![-3, -2, -1, 0, 4, 6,]
        );
        assert_eq!(
            merge_sort(&mut [1, 4, 2, 10, 23, 3, 1, 0, 20]),
            [0, 1, 1, 2, 3, 4, 10, 20, 23,]
        );
        assert_eq!(merge_sort(&mut [-3]), vec![-3]);
    }
}
