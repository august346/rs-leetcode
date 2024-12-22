fn itr10(n: i32, one: &str, five: &str, ten: &str) -> String {
    match n {
        1 => one.to_string(),
        2 => format!("{one}{one}"),
        3 => format!("{one}{one}{one}"),
        4 => format!("{one}{five}"),
        5 => five.to_string(),
        6 => format!("{five}{one}"),
        7 => format!("{five}{one}{one}"),
        8 => format!("{five}{one}{one}{one}"),
        9 => format!("{one}{ten}"),
        _ => "".to_string()
    }
}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let b10000 = "M".repeat((num / 1000) as usize);

        let mut num = num % 1000;
        let b1000 = itr10(num / 100, "C", "D", "M");

        num = num % 100;
        let b100 = itr10(num / 10, "X", "L", "C");

        num = num % 10;
        let b10 = itr10(num / 1, "I", "V", "X");

        vec![b10000, b1000, b100, b10].join("")
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::Solution;

    #[rstest]
    #[case(3749, "MMMDCCXLIX")]
    #[case(58, "LVIII")]
    #[case(1994, "MCMXCIV")]
    fn test(#[case] num: i32, #[case] expected: String) {
        let result = Solution::int_to_roman(num);
        assert_eq!(result, expected);
    }
}