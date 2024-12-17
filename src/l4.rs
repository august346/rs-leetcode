use std::cmp::{max, min};

enum MedianIndex {
    One(usize),
    Two(usize, usize)
}

struct Median {
    index: MedianIndex,
    value: f64,
}

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        fn get_median(vec: &Vec<i32>) -> Option<Median> {
            if vec.len() == 0 {
                return None
            }

            let median_index = vec.len() / 2;

            let (index, value) = match vec.len() % 2 {
                1 => {
                    let index = MedianIndex::One(median_index);
                    let value = vec[median_index] as f64;
                    (index, value)
                },
                _ => {
                    let (ind1, ind2) = (median_index - 1, median_index);
                    let index = MedianIndex::Two(ind1, ind2);
                    let value = (vec[ind1] + vec[ind2]) as f64 / 2.0;
                    (index, value)
                },
            };

            Some( Median { index, value } )
        }

        fn get_median_one_and_many(one: i32, many: &Vec<i32>) -> f64 {
            let median = get_median(&many);
            match median {
                None => 0.0,
                Some(m) => match m.index {
                    MedianIndex::One(i) => {
                        let value = many[i];
                        vec![value, match one < value {
                            true => max(one, many[i-1]),
                            false => min(one, many[i+1])
                        }].into_iter().sum::<i32>() as f64 / 2.0
                    },
                    MedianIndex::Two(i1, i2) => {
                        let mut sub = vec![many[i1], many[i2], one];
                        sub.sort();
                        get_median(&sub).unwrap().value
                    }
                }
            }
        }

        let total_len = nums1.len() + nums2.len();
        if total_len <= 5 {
            let mut new: Vec<i32> = nums1.iter().chain(nums2.iter()).cloned().collect();
            new.sort();
            return get_median(&new).unwrap().value;
        }

        let m1 = get_median(&nums1);
        let m2 = get_median(&nums2);

        match (m1, m2) {
            (Some(m1), Some(m2)) => {
                if m1.value == m2.value {
                    return m1.value
                }

                let (first, second) = match m1.value < m2.value{
                    true => ((m1, nums1), (m2, nums2)),
                    false => ((m2, nums2), (m1, nums1)),
                };

                for (a, b) in vec![(&first, &second), (&second, &first)] {
                    match a.1.len() {
                        1 => {
                            if let Some(&a_fst) = a.1.first() {
                                return get_median_one_and_many(a_fst, &b.1);
                            }
                        },
                        2 if b.1.len() >= 4 => {
                            let (a_val0, a_val1) = (a.1[0], a.1[1]);
                            let mut sub_vec = match b.0.index {
                                MedianIndex::One(b_ind) => vec![
                                    a_val0, a_val1, b.1[b_ind-1], b.1[b_ind], b.1[b_ind+1]
                                ],
                                MedianIndex::Two(b_ind1, b_ind2) => vec![
                                    a_val0, a_val1, b.1[b_ind1-1], b.1[b_ind1], b.1[b_ind2], b.1[b_ind2+1]
                                ],
                            };
                            sub_vec.sort();
                            return get_median(&sub_vec).unwrap().value;
                        },
                        _ => {}
                    }
                }

                let skip_first = match first.0.index {
                    MedianIndex::One(ind) => ind,
                    MedianIndex::Two(i1, _) => i1,
                };
                let skip_second = second.1.len() - 1 - match second.0.index {
                    MedianIndex::One(ind) => ind,
                    MedianIndex::Two(_, i2) => i2,
                };

                let to_skip = min(skip_first, skip_second);
                let new_nums1 = first.1[to_skip..].to_vec();
                let new_nums2 = second.1[..second.1.len() - to_skip].to_vec();

                Solution::find_median_sorted_arrays(new_nums1, new_nums2)
            },
            (Some(m1), None) => m1.value,
            (None, Some(m2)) => m2.value,
            _ => 0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::Solution;

    #[rstest]
    #[case(vec![1,3], vec![2], 2.0)]
    #[case(vec![1,2], vec![3,4], 2.5)]
    #[case(vec![1,2,3,4,5], vec![6,7,8,9,10,11,12,13,14,15,16,17], 9.0)]
    #[case(vec![1,2], vec![-1,3], 1.5)]
    #[case(vec![3], vec![1,2,4,5,6], 3.5)]
    #[case(vec![4], vec![1,2,3,5,6], 3.5)]
    #[case(vec![3,4,5], vec![1,2,6,7,8], 4.5)]
    #[case(vec![1,2,6,7], vec![3,4,5,8], 4.5)]
    #[case(vec![1,2], vec![3,4,5,6], 3.5)]
    fn test_length_of_longest_substring(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: f64) {
        let result = Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, expected);
    }
}

