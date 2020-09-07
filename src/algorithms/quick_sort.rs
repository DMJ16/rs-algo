pub fn quick_sort(collection: &[i32]) -> Vec<i32> {
    let mut result = collection.to_vec();
    sort(&mut result, 0, collection.len() - 1);
    result
}

fn sort(collection: &mut [i32], start: usize, end: usize) {
    if start < end {
        let pivot = partition(collection, start, end);
        sort(collection, start, pivot);
        sort(collection, pivot + 1, end);
    }
}

fn partition(collection: &mut [i32], start: usize, end: usize) -> usize {
    let pivot = collection[end].clone();
    let (mut i, mut j) = (start as isize - 1, end as isize + 1);

    loop {
        'lower: loop {
            i += 1;
            if i > j || collection[i as usize] >= pivot {
                break 'lower;
            }
        }

        'upper: loop {
            j -= 1;
            if i > j || collection[j as usize] <= pivot {
                break 'upper;
            }
        }

        if i > j {
            return j as usize;
        }

        collection.swap(i as usize, j as usize);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quick_sort_test() {
        assert_eq!(quick_sort(&[10, 5, 2, 3]), vec![2, 3, 5, 10]);
        assert_eq!(quick_sort(&[11, 7, 1, 14]), vec![1, 7, 11, 14]);
        assert_eq!(quick_sort(&[3, 1, 7, 11]), vec![1, 3, 7, 11]);
        assert_eq!(quick_sort(&[100, 200, 300, 400]), vec![100, 200, 300, 400]);
        assert_eq!(
            quick_sort(&[-3, 4, 0, -2, 6, -1]),
            vec![-3, -2, -1, 0, 4, 6,]
        );
        assert_eq!(
            quick_sort(&[1, 4, 2, 10, 23, 3, 1, 0, 20]),
            vec![0, 1, 1, 2, 3, 4, 10, 20, 23,]
        );
        assert_eq!(quick_sort(&[-3]), vec![-3]);
    }
}
