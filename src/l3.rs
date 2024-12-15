use std::cmp::max;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut current = HashMap::new();
        let mut start = 0;
        let mut max_length = 0;

        for (ind, c) in s.chars().enumerate() {
            if let Some(old_ind) = current.get(&c) {
                start = max(start, old_ind + 1);
            }
            current.insert(c, ind);
            max_length = max(max_length, ind - start + 1)
        }

        max_length as i32
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::Solution;

    #[rstest]
    #[case("abcabcbb", 3)] // The substring is "abc".
    #[case("bbbbb", 1)]    // The substring is "b".
    #[case("pwwkew", 3)]   // The substring is "wke".
    #[case("", 0)]         // Empty string case.
    #[case("a", 1)]        // Single character string.
    #[case("abcdef", 6)]   // No repeating characters.
    #[case("abba", 2)]     // The substring is "ab" or "ba".
    #[case("tmmzuxt", 5)]  // The substring is "mzuxt".
    #[case("dvdf", 3)]  // The substring is "mzuxt".
    fn test_length_of_longest_substring(#[case] s: &str, #[case] expected: i32) {
        let result = Solution::length_of_longest_substring(s.to_string());
        assert_eq!(result, expected);
    }
}

