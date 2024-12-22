use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut height: Vec<(usize, i32)> = height.into_iter().enumerate().collect();
        height.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let mut height = height.into_iter().rev();
        if let Some(first) = height.next() {
            if let Some(second) = height.next() {
                let (mut left, mut right) = match first.0 < second.0 {
                    true => (first, second),
                    false => (second, first),
                };

                loop {
                    result = max(result, (right.0 - left.0) as i32 * min(left.1, right.1));

                    (left, right) = match height.next() {
                        None => break,
                        Some(next) => match next.0 {
                            ind if ind < left.0 => (next, right),
                            ind if ind > right.0 => (left, next),
                            _ => (left, right)
                        }
                    };
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::Solution;

    #[rstest]
    #[case(vec![1,8,6,2,5,4,8,3,7], 49)]
    #[case(vec![1,1], 1)]
    fn test(#[case] height: Vec<i32>, #[case] expected: i32) {
        let result = Solution::max_area(height);
        assert_eq!(result, expected);
    }
}