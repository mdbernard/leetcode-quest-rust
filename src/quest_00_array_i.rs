pub struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::from(&nums[..]);
        ans.extend(&nums[..]);
        return ans;
    }

    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = usize::try_from(n).expect("n must be a valid `usize`");
        let mut ans = vec![];
        for i in 0..n {
            ans.push(nums[i]);
            ans.push(nums[i + n]);
        }
        return ans;
    }

    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut current = 0;

        for n in nums.iter() {
            if *n == 1 {
                current += 1;
                if current > ans {
                    ans = current;
                }
            } else {
                current = 0;
            }
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_concatenation_example_1() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 2, 1]),
            vec![1, 2, 1, 1, 2, 1]
        );
    }

    #[test]
    fn get_concatenation_example_2() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 3, 2, 1]),
            vec![1, 3, 2, 1, 1, 3, 2, 1]
        );
    }

    #[test]
    fn shuffle_example_1() {
        assert_eq!(
            Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3),
            vec![2, 3, 5, 4, 1, 7]
        );
    }

    #[test]
    fn shuffle_example_2() {
        assert_eq!(
            Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1]
        );
    }

    #[test]
    fn shuffle_example_3() {
        assert_eq!(Solution::shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
    }

    #[test]
    fn find_max_consecutive_ones_example_1() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
    }

    #[test]
    fn find_max_consecutive_ones_example_2() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        );
    }
}
