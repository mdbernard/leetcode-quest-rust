pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut sum = digits.clone();
        let mut carry = 1;
        for (rev_i, digit) in digits.iter().rev().enumerate() {
            let i = digits.len() - rev_i - 1;
            let new = if *digit + carry >= 10 {
                carry = *digit + carry - 9;
                0
            } else {
                let val = *digit + carry;
                carry = 0;
                val
            };
            sum[i] = new;
        }
        if carry != 0 {
            let mut final_sum = vec![carry];
            final_sum.extend(sum.iter());
            final_sum
        } else {
            sum
        }
    }

    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        if !(arr[1] > arr[0]) {
            return false;
        }

        let mut found_peak = false;
        let mut prev_value = arr[0];
        for (i, value) in arr.iter().enumerate() {
            if i == 0 {
                continue;
            }

            if *value == prev_value {
                return false;
            }

            if *value > prev_value {
                if !found_peak {
                    prev_value = *value;
                    continue;
                } else {
                    return false;
                }
            } else {
                if found_peak {
                    prev_value = *value;
                    continue;
                } else {
                    prev_value = *value;
                    found_peak = true;
                    continue;
                }
            }
        }

        if !found_peak {
            return false;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_one_example_1() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4])
    }

    #[test]
    fn plus_one_example_2() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2])
    }

    #[test]
    fn plus_one_example_3() {
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0])
    }

    #[test]
    fn valid_mountain_array_example_1() {
        assert_eq!(Solution::valid_mountain_array(vec![2, 1]), false)
    }

    #[test]
    fn valid_mountain_array_example_2() {
        assert_eq!(Solution::valid_mountain_array(vec![3, 3, 5]), false)
    }

    #[test]
    fn valid_mountain_array_example_3() {
        assert_eq!(Solution::valid_mountain_array(vec![0, 3, 2, 1]), true)
    }

    #[test]
    fn valid_mountain_array_example_4() {
        assert_eq!(
            Solution::valid_mountain_array(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            false
        )
    }

    #[test]
    fn valid_mountain_array_example_5() {
        assert_eq!(
            Solution::valid_mountain_array(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
            false
        )
    }
}
