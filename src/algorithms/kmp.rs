pub fn kmp(str: &str, substr: &str) -> usize {
    let mut count = 0;
    let mut i = 0;
    let mut j = substr.len();

    while j <= str.len() {
        if substr == &str[i..j] {
            count += 1;
        }
        i += 1;
        j += 1;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn kmp_test() {
        assert_eq!(kmp("hannahhahhahahahhahahahannah", "hannah"), 2);
        assert_eq!(kmp("hannahhahhahahahhahahahannah", "dsds"), 0);
        assert_eq!(kmp("hannahhahhahahahhahahahannah", "ha"), 9);
        assert_eq!(kmp("hannahhahhahahah hahah     ahannah", "hannah"), 2);
    }
}
