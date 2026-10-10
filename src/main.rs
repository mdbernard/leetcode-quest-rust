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
