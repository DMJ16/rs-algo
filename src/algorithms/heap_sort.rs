use std::collections::BinaryHeap;

pub fn heap_sort<T: Ord + Copy>(collection: &[T]) -> Vec<T> {
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
        let mut v1 = vec![10, 5, 2, 3];
        let mut v2 = vec![11, 7, 1, 14];
        let mut v3 = vec![3, 1, 7, 11];
        let mut v4 = vec![100, 200, 300, 400];
        let mut v5 = vec![-3, 4, 0, -2, 6, -1];
        let mut v6 = vec![1, 4, 2, 10, 23, 3, 1, 0, 20];
        let mut v7 = vec![-3];
        v1 = heap_sort(&v1);
        v2 = heap_sort(&v2);
        v3 = heap_sort(&v3);
        v4 = heap_sort(&v4);
        v5 = heap_sort(&v5);
        v6 = heap_sort(&v6);
        v7 = heap_sort(&v7);

        assert_eq!(v1, vec![2, 3, 5, 10]);
        assert_eq!(v2, vec![1, 7, 11, 14]);
        assert_eq!(v3, vec![1, 3, 7, 11]);
        assert_eq!(v4, vec![100, 200, 300, 400]);
        assert_eq!(v5, vec![-3, -2, -1, 0, 4, 6,]);
        assert_eq!(v6, [0, 1, 1, 2, 3, 4, 10, 20, 23,]);
        assert_eq!(v7, vec![-3]);
    }
}
