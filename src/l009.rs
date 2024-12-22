use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut deque: VecDeque<char> = VecDeque::from(format!("{}", x).chars().collect::<Vec<char>>());

        while !deque.is_empty() {
            if let Some(front) = deque.pop_front() {
                if let Some(back) = deque.pop_back() {
                    if front != back {
                        return false;
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::Solution;

    #[rstest]
    #[case(121, true)]
    #[case(-121, false)]
    #[case(10, false)]
    fn test(#[case] x: i32, #[case] expected: bool) {
        let result = Solution::is_palindrome(x);
        assert_eq!(result, expected);
    }
}
