pub fn factorial_iter(num: u32) -> u32 {
    (1..=num).product()
}

pub fn factorial_recur(num: u32) -> u32 {
    match num {
        0 | 1 => 1,
        _ => num * factorial_recur(num - 1),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn factorial_iterative() {
        assert_eq!(factorial_iter(0), 1);
        assert_eq!(factorial_iter(1), 1);
        assert_eq!(factorial_iter(2), 2);
        assert_eq!(factorial_iter(3), 6);
        assert_eq!(factorial_iter(4), 24);
        assert_eq!(factorial_iter(5), 120);
        assert_eq!(factorial_iter(6), 720);
    }

    #[test]
    fn factorial_recursive() {
        assert_eq!(factorial_recur(0), 1);
        assert_eq!(factorial_recur(1), 1);
        assert_eq!(factorial_recur(2), 2);
        assert_eq!(factorial_recur(3), 6);
        assert_eq!(factorial_recur(4), 24);
        assert_eq!(factorial_recur(5), 120);
        assert_eq!(factorial_recur(6), 720);
    }
}
