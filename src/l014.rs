impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = "".to_string();

        let iter_chars: Vec<Vec<char>> = strs.into_iter().map(|s| s.chars().collect()).collect();
        let mut index = 0;

        let mut round_chars: Vec<_> = iter_chars.iter().map(|cs| cs.get(index)).collect();
        loop {
            let mut stop = false;

            match round_chars.first() {
                Some(Some(&first)) => {
                    for c in round_chars {
                        match c {
                            Some(&c) if c == first => {},
                            _ => {
                                stop = true;
                                break;
                            }
                        }
                    }
                    if !stop {
                        result = format!("{result}{first}")
                    }
                },
                _ => stop = true,
            };

            if stop {
                break
            }

            index += 1;
            round_chars = iter_chars.iter().map(|cs| cs.get(index)).collect();
        }

        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::Solution;

    #[rstest]
    #[case(vec!["flower","flow","flight"], "fl")]
    #[case(vec!["dog","racecar","car"], "")]
    #[case(vec!["ab","a"], "a")]
    fn test(#[case] strs: Vec<&str>, #[case] expected: String) {
        let strs = strs.iter().map(|s| s.to_string()).collect();
        let result = Solution::longest_common_prefix(strs);
        assert_eq!(result, expected);
    }
}