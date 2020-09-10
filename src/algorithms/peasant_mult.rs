pub fn peasant_multiplication(x: u32, y: u32) -> u32 {
    if x == 0 {
        return 0;
    }
    let x_prime = x / 2;
    let y_prime = y + y;
    let mut product = peasant_multiplication(x_prime, y_prime);

    if x % 2 != 0 {
        product += y;
    }
    product
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_peasant_multiplication() {
        assert_eq!(peasant_multiplication(2, 3), 6);
        assert_eq!(peasant_multiplication(1, 4), 4);
        assert_eq!(peasant_multiplication(5, 30), 150);
        assert_eq!(peasant_multiplication(2, 100), 200);
    }
}
