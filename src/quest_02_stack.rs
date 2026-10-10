pub struct Solution;

struct ExclusiveTime {
    id: i32,
    start_time: i32,
    time_out: i32,
}

impl Solution {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn owned(tokens: &[&str]) -> Vec<String> {
        tokens.iter().map(|s| (*s).to_string()).collect()
    }

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
