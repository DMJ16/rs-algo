use std::cmp::max;

pub fn kadane(vec: Vec<i32>) -> i32 {
    let mut max_at_idx = vec[0];
    let mut current_max = vec[0];

    for i in 1..vec.len() {
        let num = vec[i];
        max_at_idx = max(num, max_at_idx + num);
        current_max = max(current_max, max_at_idx);
    }

    current_max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kadane() {
        assert_eq!(
            kadane(vec![3, 5, -9, 1, 3, -2, 3, 4, 7, 2, -9, 6, 3, 1, -5, 4]),
            19
        );
        assert_eq!(kadane(vec![-10, -2, -9, -4, -8, -6, -7, -1, -3, -5]), -1);
    }
}
