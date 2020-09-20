pub fn sieve_of_e(bound: usize) -> Vec<usize> {
    let mut prime = vec![true; bound + 1];
    prime[0] = false;
    if bound >= 1 {
        prime[1] = false;
    }

    for num in 2..bound + 1 {
        if prime[num] {
            let mut multiple = num * num;
            while multiple <= bound {
                prime[multiple] = false;
                multiple += num;
            }
        }
    }

    prime
        .iter()
        .enumerate()
        .filter_map(|(prime, &is_prime)| if is_prime { Some(prime) } else { None })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_primes() {
        assert_eq!(sieve_of_e(5), vec![2, 3, 5]);
        assert_eq!(sieve_of_e(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
        assert_eq!(
            sieve_of_e(100),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }
}
