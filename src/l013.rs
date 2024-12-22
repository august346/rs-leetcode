fn int(s: &str) -> i32 {
    match s {
        "M" => 1000,
        "D" => 500,
        "C" => 100,
        "L" => 50,
        "X" => 10,
        "V" => 5,
        "I" => 1,
        _ => 0,
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut prev = 10_000;

        for c in s.chars() {
            let c_str = &c.to_string();
            let now = int(c_str);
            result += match prev >= now {
                true => now,
                false => now - prev - prev,
            };
            prev = now;
        }

        result
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::Solution;

    #[rstest]
    #[case("MMMDCCXLIX", 3749)]
    #[case("LVIII", 58)]
    #[case("MCMXCIV", 1994)]
    #[case("III", 3)]
    fn test(#[case] s: String, #[case] expected: i32) {
        let result = Solution::roman_to_int(s);
        assert_eq!(result, expected);
    }
}
