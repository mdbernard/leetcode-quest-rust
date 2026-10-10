use std::collections::HashSet;

struct Solution;

struct ExclusiveTime {
    id: i32,
    start_time: i32,
    time_out: i32,
}

impl Solution {
    // ------------------------------------------------------------------------
    // ARRAY I
    // ------------------------------------------------------------------------

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

    // ------------------------------------------------------------------------
    // ARRAY II
    // ------------------------------------------------------------------------

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

    // ------------------------------------------------------------------------
    // STACK
    // ------------------------------------------------------------------------

    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut ans = vec![];
        let push = String::from("Push");
        let pop = String::from("Pop");

        let mut stack: Vec<i32> = vec![];
        let mut stream = 1;

        for &number in target.iter() {
            if stack == target {
                break;
            }
            while number != stream {
                ans.push(push.clone());
                ans.push(pop.clone());
                stream += 1;
            }
            stack.push(number);
            ans.push(push.clone());
            stream += 1;
        }

        return ans;
    }

    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];

        for token in &tokens {
            match token.as_str() {
                "+" => {
                    let new = stack.pop().unwrap() + stack.pop().unwrap();
                    stack.push(new);
                }
                "-" => {
                    let new = -stack.pop().unwrap() + stack.pop().unwrap();
                    stack.push(new);
                }
                "*" => {
                    let new = stack.pop().unwrap() * stack.pop().unwrap();
                    stack.push(new);
                }
                "/" => {
                    let denominator = stack.pop().unwrap();
                    let numerator = stack.pop().unwrap();
                    stack.push(numerator / denominator);
                }
                _ => stack.push(token.parse::<i32>().unwrap()),
            }
        }

        return stack.pop().unwrap();
    }

    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0; n as usize];
        let mut stack: Vec<ExclusiveTime> = vec![];

        for log in &logs {
            let mut parts = log.split(":");
            let id = parts.next().unwrap().parse::<i32>().unwrap();
            let start_end = parts.next().unwrap();
            let time = parts.next().unwrap().parse::<i32>().unwrap();

            if start_end == "start" {
                stack.push(ExclusiveTime {
                    id,
                    start_time: time,
                    time_out: 0,
                });
                continue;
            }

            let start_log = stack.pop().unwrap();
            let total_time = (time + 1) - start_log.start_time;
            let exclusive_time = total_time - start_log.time_out;
            ans[start_log.id as usize] += exclusive_time;

            let top = stack.last_mut();
            if top.is_some() {
                top.unwrap().time_out += total_time;
            }
        }
        return ans;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn owned(tokens: &[&str]) -> Vec<String> {
        tokens.iter().map(|s| (*s).to_string()).collect()
    }

    // ------------------------------------------------------------------------
    // ARRAY I
    // ------------------------------------------------------------------------

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

    // ------------------------------------------------------------------------
    // ARRAY II
    // ------------------------------------------------------------------------

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

    // ------------------------------------------------------------------------
    // STACK
    // ------------------------------------------------------------------------

    #[test]
    fn build_array_example_1() {
        assert_eq!(
            Solution::build_array(vec![1, 3], 3),
            owned(&["Push", "Push", "Pop", "Push"])
        );
    }

    #[test]
    fn build_array_example_2() {
        assert_eq!(
            Solution::build_array(vec![1, 2, 3], 3),
            owned(&["Push", "Push", "Push"])
        );
    }

    #[test]
    fn build_array_example_3() {
        assert_eq!(
            Solution::build_array(vec![1, 2], 4),
            owned(&["Push", "Push"])
        );
    }

    #[test]
    fn eval_rpn_example_1() {
        assert_eq!(Solution::eval_rpn(owned(&["2", "1", "+", "3", "*"])), 9);
    }

    #[test]
    fn eval_rpn_example_2() {
        assert_eq!(Solution::eval_rpn(owned(&["4", "13", "5", "/", "+"])), 6);
    }

    #[test]
    fn eval_rpn_example_3() {
        assert_eq!(
            Solution::eval_rpn(owned(&[
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
            ])),
            22
        );
    }

    #[test]
    fn exclusive_time_example_1() {
        assert_eq!(
            Solution::exclusive_time(2, owned(&["0:start:0", "1:start:2", "1:end:5", "0:end:6"])),
            vec![3, 4]
        );
    }

    #[test]
    fn exclusive_time_example_2() {
        assert_eq!(
            Solution::exclusive_time(
                1,
                owned(&[
                    "0:start:0",
                    "0:start:2",
                    "0:end:5",
                    "0:start:6",
                    "0:end:6",
                    "0:end:7"
                ])
            ),
            vec![8]
        );
    }

    #[test]
    fn exclusive_time_example_3() {
        assert_eq!(
            Solution::exclusive_time(
                2,
                owned(&[
                    "0:start:0",
                    "0:start:2",
                    "0:end:5",
                    "1:start:6",
                    "1:end:6",
                    "0:end:7"
                ])
            ),
            vec![7, 1]
        );
    }
}
