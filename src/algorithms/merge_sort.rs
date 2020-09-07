pub fn merge_sort(collection: &[i32]) -> Vec<i32> {
    if collection.len() > 1 {
        let (left_slice, right_slice) = collection.split_at(collection.len() / 2);
        let left = merge_sort(left_slice);
        let right = merge_sort(right_slice);
        let mut result: Vec<i32> = collection.to_vec();
        let (mut i, mut j, mut k) = (0, 0, 0);

        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                result[k] = left[i];
                i += 1;
            } else {
                result[k] = right[j];
                j += 1;
            }
            k += 1;
        }

        while i < left.len() {
            result[k] = left[i];
            k += 1;
            i += 1;
        }

        while j < right.len() {
            result[k] = right[j];
            k += 1;
            j += 1;
        }

        result
    } else {
        collection.to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn merge_sort_test() {
        assert_eq!(merge_sort(&[10, 5, 2, 3]), vec![2, 3, 5, 10]);
        assert_eq!(merge_sort(&[11, 7, 1, 14]), vec![1, 7, 11, 14]);
        assert_eq!(merge_sort(&[3, 1, 7, 11]), vec![1, 3, 7, 11]);
        assert_eq!(merge_sort(&[100, 200, 300, 400]), vec![100, 200, 300, 400]);
        assert_eq!(
            merge_sort(&[-3, 4, 0, -2, 6, -1]),
            vec![-3, -2, -1, 0, 4, 6,]
        );
        assert_eq!(
            merge_sort(&[1, 4, 2, 10, 23, 3, 1, 0, 20]),
            [0, 1, 1, 2, 3, 4, 10, 20, 23,]
        );
        assert_eq!(merge_sort(&[-3]), vec![-3]);
    }
}
