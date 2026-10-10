use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut duplicate = None;
        let all = HashSet::<i32>::from_iter(1..=nums.len().try_into().unwrap());
        let mut seen = HashSet::<i32>::new();
        for n in nums.iter() {
            if seen.contains(n) {
                duplicate = Some(*n);
            } else {
                seen.insert(*n);
            }
        }
        let mut remainder = all.difference(&seen);
        let missing = match (remainder.next(), remainder.next()) {
            (Some(&n), None) => Some(n),
            _ => panic!("Expected exactly 1 missing value"),
        };
        return vec![duplicate.unwrap(), missing.unwrap()];
    }

    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        for i in 0..nums.len() {
            let mut num_less = 0;
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }
                if nums[j] < nums[i] {
                    num_less += 1;
                }
            }
            ans.push(num_less);
        }
        return ans;
    }

    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut exists = vec![false; nums.len()];
        for n in &nums {
            exists[*n as usize - 1] = true;
        }
        exists
            .iter()
            .enumerate()
            .filter_map(|(i, &num_exists)| (!num_exists).then_some((i + 1) as i32))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_error_nums_example_1() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    }

    #[test]
    fn find_error_nums_example_2() {
        assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
    }

    #[test]
    fn smaller_numbers_than_current_example_1() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
    }

    #[test]
    fn smaller_numbers_than_current_example_2() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
    }

    #[test]
    fn smaller_numbers_than_current_example_3() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }

    #[test]
    fn find_disappeared_numbers_example_1() {
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
    }

    #[test]
    fn find_disappeared_numbers_example_2() {
        assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2]);
    }
}
