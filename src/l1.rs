struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() < 2 {
            return vec![];
        }

        let mut pairs: Vec<(usize, i32)> = nums
            .iter()
            .enumerate()
            .map(|(index, &num)| (index, num))
            .collect();
        pairs.sort_by(|a, b| a.1.cmp(&b.1));

        let (mut left_ind, mut right_ind) = (0, nums.len() - 1);

        while left_ind < right_ind {
            let left_item = pairs[left_ind];
            while right_ind > left_ind {
                let right_item = pairs[right_ind];
                let sum = left_item.1 + right_item.1;
                if sum == target {
                    let mut result = vec![left_item.0 as i32, right_item.0 as i32];
                    result.sort();
                    return result;
                } else if sum < target {
                    break
                }
                right_ind -= 1;
            }
            left_ind += 1;
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::Solution;

    #[rstest]
    #[case(vec![2, 7, 11, 15], 9, vec![0, 1])]
    #[case(vec![3, 2, 4], 6, vec![1, 2])]
    #[case(vec![3, 3], 6, vec![0, 1])]
    #[case(vec![1, 2, 3], 7, vec![])] // No solution case
    #[case(vec![], 5, vec![])]        // Empty array case
    #[case(vec![1, 6142, 8192, 10239], 18431, vec![2, 3])]
    #[case(vec![5, 75, 25], 100, vec![1, 2])]
    fn test_two_sum(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: Vec<i32>) {
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, expected);
    }
}
