struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut storage = vec![];
        for _ in 0..num_rows {
            storage.push(vec![]);
        };

        let mut up = true;
        let mut row = 1;
        for c in s.chars() {
            if (!up && (row == (num_rows as usize - 1)))
                || (up && (row == 0)) {
                up = !up;
            };

            match up {
                true => row -= 1,
                false => row +=1,
            }

            storage[row].push(c as u8);
        }

        let mut result = vec![];
        for s_row in storage {
            result.extend(s_row);
        }

        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::Solution;

    #[rstest]
    #[case("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR")]
    #[case("PAYPALISHIRING", 4, "PINALSIGYAHRPI")]
    #[case("AB", 1, "AB")]
    fn test(#[case] s: String, #[case] num_rows: i32, #[case] expected: String) {
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, expected);
    }
}

