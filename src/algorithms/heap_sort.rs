use std::collections::BinaryHeap;

pub fn heap_sort(collection: &[i32]) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    for item in collection {
        heap.push(*item);
    }
    heap.into_sorted_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn heap_sort_test() {
        assert_eq!(heap_sort(&[10, 5, 2, 3]), vec![2, 3, 5, 10]);
        assert_eq!(heap_sort(&[11, 7, 1, 14]), vec![1, 7, 11, 14]);
        assert_eq!(heap_sort(&[3, 1, 7, 11]), vec![1, 3, 7, 11]);
        assert_eq!(heap_sort(&[100, 200, 300, 400]), vec![100, 200, 300, 400]);
        assert_eq!(
            heap_sort(&[-3, 4, 0, -2, 6, -1]),
            vec![-3, -2, -1, 0, 4, 6,]
        );
        assert_eq!(
            heap_sort(&[1, 4, 2, 10, 23, 3, 1, 0, 20]),
            [0, 1, 1, 2, 3, 4, 10, 20, 23,]
        );
        assert_eq!(heap_sort(&[-3]), vec![-3]);
    }
}
