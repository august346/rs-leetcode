struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut numbers = vec![];
        let k = match x < 0 {
            true => -1,
            false => 1
        };
        let mut x = x.abs();
        while x > 0 {
            numbers.push(x % 10);
            x /= 10;
        }

        let mut result = 0;
        for (ind, n) in numbers.into_iter().rev().enumerate() {
            let prev = result;
            match 10_i32
                .checked_pow(ind as u32)
                .and_then(|val| val.checked_mul(n))
                .and_then(|val| val.checked_mul(k)) {
                Some(p) => result += p,
                None => return 0
            }

            match k > 0 {
                true if prev > result => return 0,
                false if prev < result => return 0,
                _ => {}
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
    #[case(123, 321)]
    #[case(-123, -321)]
    #[case(120, 21)]
    #[case(1534236469, 0)]
    fn test(#[case] x: i32, #[case] expected: i32) {
        let result = Solution::reverse(x);
        assert_eq!(result, expected);
    }
}

