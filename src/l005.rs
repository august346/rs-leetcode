struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        fn is_palindrome(sub: &str) -> bool {
            let mut front = sub.chars();
            let mut back = sub.chars().rev();
            while let (Some(a), Some(b)) = (front.next(), back.next()) {
                if a != b {
                    return false;
                }
            }

            true
        }

        let s_len = s.len();
        let mut left_ind = 0;
        let (mut max_len, mut word) = (0, "");
        while left_ind < s_len && (s_len - left_ind) > max_len {
            let mut right_ind = s_len - 1;
            while left_ind <= right_ind {
                let new_len = right_ind - left_ind + 1;
                if new_len <= max_len {
                    break
                }
                let new_word = &s[left_ind..right_ind+1];
                if is_palindrome(new_word) {
                    (max_len, word) = (new_len, new_word);
                    break
                }
                right_ind -= 1;
            }
            left_ind += 1;
        }

        String::from(word)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::Solution;

    #[rstest]
    #[case("babad", "bab")]
    #[case("cbbd", "bb")]
    #[case("a", "a")]
    fn test(#[case] s: String, #[case] expected: String) {
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, expected);
    }
}

