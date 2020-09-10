pub fn levenshtein_dist(str1: &str, str2: &str) -> usize {
    let mut result = 0;

    if str1 == str2 {
        return result;
    }
    let length_str1 = str1.chars().count();
    let length_str2 = str2.chars().count();

    if length_str1 == 0 {
        return length_str2;
    }

    if length_str2 == 0 {
        return length_str1;
    }

    let mut cache: Vec<usize> = vec![0; length_str1];
    let mut str1_idx = 0;
    let mut distance_str1;
    let mut distance_str2;

    while str1_idx < length_str1 {
        str1_idx += 1;
        cache[str1_idx - 1] = str1_idx;
    }

    for (str2_idx, code_str2) in str2.chars().enumerate() {
        result = str2_idx;
        distance_str1 = str2_idx;

        for (str1_idx, code_a) in str1.chars().enumerate() {
            distance_str2 = if code_a == code_str2 {
                distance_str1
            } else {
                distance_str1 + 1
            };

            distance_str1 = cache[str1_idx];

            result = if distance_str1 > result {
                if distance_str2 > result {
                    result + 1
                } else {
                    distance_str2
                }
            } else if distance_str2 > distance_str1 {
                distance_str1 + 1
            } else {
                distance_str2
            };

            cache[str1_idx] = result;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn levenshtein() {
        assert_eq!(levenshtein_dist("abc", "yabd"), 2);
        assert_eq!(levenshtein_dist("abc", "abc"), 0);
        assert_eq!(levenshtein_dist("xabc", "abcx"), 2);
    }
}
