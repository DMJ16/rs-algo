use std::collections::HashMap;

pub fn fib_exp(n: u32) -> u32 {
    let sqrt_5 = 5.0f64.sqrt();
    let phi = (1.0 + sqrt_5) / 2.0;
    let q = 1.0 / phi;
    ((phi.powi(n as i32) + q.powi(n as i32)) / sqrt_5 + 0.5) as u32
}

pub fn fib_iter(n: u32) -> u32 {
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

pub fn fib_recur(n: u32, memo: &mut HashMap<u32, u32>) -> u32 {
    match memo.get(&n) {
        Some(result) => *result,
        None => {
            let result = match n {
                0 | 1 => n,
                n => fib_recur(n - 1, memo) + fib_recur(n - 2, memo),
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
    fn get_nth_fibonacci_expotential() {
        assert_eq!(fib_exp(1), 1);
        assert_eq!(fib_exp(2), 1);
        assert_eq!(fib_exp(3), 2);
        assert_eq!(fib_exp(4), 3);
        assert_eq!(fib_exp(5), 5);
        assert_eq!(fib_exp(6), 8);
    }
    #[test]
    fn get_nth_fibonacci_iterative() {
        assert_eq!(fib_iter(0), 0);
        assert_eq!(fib_iter(1), 1);
        assert_eq!(fib_iter(2), 1);
        assert_eq!(fib_iter(3), 2);
        assert_eq!(fib_iter(4), 3);
        assert_eq!(fib_iter(5), 5);
    }
    #[test]
    fn get_nth_fibonacci_recursive() {
        let mut hashmap = HashMap::new();
        assert_eq!(fib_recur(0, &mut hashmap), 0);
        assert_eq!(fib_recur(1, &mut hashmap), 1);
        assert_eq!(fib_recur(2, &mut hashmap), 1);
        assert_eq!(fib_recur(3, &mut hashmap), 2);
        assert_eq!(fib_recur(4, &mut hashmap), 3);
        assert_eq!(fib_recur(5, &mut hashmap), 5);
    }
}
