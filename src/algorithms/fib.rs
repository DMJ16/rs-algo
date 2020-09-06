use std::collections::HashMap;

pub fn fib_iterative(n: usize) -> usize {
    let mut memo = (0, 1);
    match n {
        0 | 1 => n,
        _ => {
            for _ in 2..=n {
                memo = (memo.1, memo.0 + memo.1)
            }
            memo.1
        }
    }
}

pub fn fib_recursive(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    match memo.get(&n).copied() {
        Some(result) => result,
        None => {
            let result = match n {
                0 | 1 => n,
                n => fib_recursive(n - 1, memo) + fib_recursive(n - 2, memo),
            };
            memo.insert(n, result);
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_nth_fibonacci_iterative_dp() {
        assert_eq!(fib_iterative(0), 0);
        assert_eq!(fib_iterative(1), 1);
        assert_eq!(fib_iterative(2), 1);
        assert_eq!(fib_iterative(3), 2);
        assert_eq!(fib_iterative(4), 3);
        assert_eq!(fib_iterative(5), 5);
    }
    #[test]
    fn get_nth_fibonacci_recursive_memo() {
        let mut hashmap: HashMap<usize, usize> = HashMap::new();
        assert_eq!(fib_recursive(0, &mut hashmap), 0);
        assert_eq!(fib_recursive(1, &mut hashmap), 1);
        assert_eq!(fib_recursive(2, &mut hashmap), 1);
        assert_eq!(fib_recursive(3, &mut hashmap), 2);
        assert_eq!(fib_recursive(4, &mut hashmap), 3);
        assert_eq!(fib_recursive(5, &mut hashmap), 5);
    }
}
