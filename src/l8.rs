use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        fn as_char(c_str: &str) -> char {
            c_str.chars().next().unwrap()
        }

        let (space, dash, plus) = (as_char(" "), as_char("-"), as_char("+"));

        let mut allowed_symbols = HashMap::new();
        allowed_symbols.insert(dash, -1);
        for i in 0..10 {
            allowed_symbols.insert(as_char(&format!("{}", i)), i);
        }

        let mut k = None;
        let mut numbers = None;
        for c in s.chars() {
            if c == space {
                match numbers.is_none() && k.is_none() {
                    true => continue,
                    false => break
                }
            }

            if c == dash || c == plus {
                match numbers.is_none() {
                    true => {
                        match k.is_none() {
                            true => {
                                k = Some(if c == dash {-1} else {1}) ;
                                continue;
                            },
                            false => break
                        }
                    },
                    false => break,
                }
            }
            match allowed_symbols.get(&c) {
                Some(&x) => {
                    match numbers {
                        None => numbers = Some(match x == 0 {
                            true => vec![],
                            false => vec![x],
                        }),
                        Some(ref mut numbers) if !(x==0 && numbers.is_empty()) => numbers.push(x),
                        _ => {}
                    }
                },
                None => break
            }
        }

        let mut result = 0;
        let (min, max) = (i32::MIN, i32::MAX);
        let k = k.unwrap_or(1);
        for (ind, &n) in numbers.unwrap_or(vec![]).iter().rev().enumerate() {
            let prev = result;
            match 10_i32
                .checked_pow(ind as u32)
                .and_then(|val| val.checked_mul(n))
                .and_then(|val| val.checked_mul(k)) {
                Some(p) => result += p,
                None => return match k == -1 {
                    true => min,
                    false => max
                }
            }
            match k == 1 {
                true if prev > result => return max,
                false if prev < result => return min,
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
    #[case("42", 42)]
    #[case("-4", -4)]
    #[case(" -4", -4)]
    #[case(" -04", -4)]
    #[case(" -042", -42)]
    #[case("1337c0d3", 1337)]
    #[case("0-1", 0)]
    #[case("words and 987", 0)]
    #[case("-91283472332", -2147483648)]
    #[case("+1", 1)]
    #[case("  0000000000012345678", 12345678)]
    fn test(#[case] s: String, #[case] expected: i32) {
        let result = Solution::my_atoi(s);
        assert_eq!(result, expected);
    }
}
