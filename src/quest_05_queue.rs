use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students = VecDeque::from(students);
        let mut sandwiches = VecDeque::from(sandwiches);

        let mut num_skipped = 0;
        while !students.is_empty() && num_skipped < students.len() {
            // SAFETY: already ensured `students` is not empty, and `sandwiches` is same length as `students`
            if students.front().unwrap() == sandwiches.front().unwrap() {
                students.pop_front();
                sandwiches.pop_front();
                num_skipped = 0;
            } else {
                let first = students.pop_front().unwrap();
                students.push_back(first);
                num_skipped += 1;
            }
        }
        students.len() as i32
    }

    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut tickets = VecDeque::from(tickets);
        let mut k = k as usize;
        let mut time = 0;

        while let Some(first) = tickets.pop_front() {
            time += 1;
            let remaining = first - 1;
            if k == 0 && remaining == 0 {
                break;
            }
            if remaining > 0 {
                tickets.push_back(remaining);
            }
            k = if k == 0 { tickets.len() - 1 } else { k - 1 }
        }
        time
    }
}

#[allow(dead_code)]
struct MyQueue {
    contents: Vec<i32>,
}

#[allow(dead_code)]
impl MyQueue {
    fn new() -> Self {
        MyQueue { contents: vec![] }
    }

    fn push(&mut self, x: i32) {
        self.contents.push(x);
    }

    fn pop(&mut self) -> i32 {
        let mut reverse: Vec<i32> = vec![];
        while self.contents.len() > 1 {
            // SAFETY: verified `contents` has at least one item
            reverse.push(self.contents.pop().unwrap());
        }
        // SAFETY: `contents` guaranteed to have exactly one item
        let return_value = self.contents.pop().unwrap();
        while let Some(item) = reverse.pop() {
            self.contents.push(item);
        }
        return_value
    }

    fn peek(&mut self) -> i32 {
        let mut reverse: Vec<i32> = vec![];
        while self.contents.len() > 1 {
            // SAFETY: verified `contents` has at least one item
            reverse.push(self.contents.pop().unwrap());
        }
        // SAFETY: `contents` guaranteed to have exactly one item
        let return_value = self.contents.pop().unwrap();
        reverse.push(return_value);
        while let Some(item) = reverse.pop() {
            self.contents.push(item);
        }
        return_value
    }

    fn empty(&self) -> bool {
        self.contents.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_students_example_1() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
            0
        )
    }

    #[test]
    fn count_students_example_2() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        )
    }

    #[test]
    fn time_required_to_buy_example_1() {
        assert_eq!(Solution::time_required_to_buy(vec![2, 3, 2], 2), 6)
    }

    #[test]
    fn time_required_to_buy_example_2() {
        assert_eq!(Solution::time_required_to_buy(vec![5, 1, 1, 1], 0), 8)
    }

    #[test]
    fn my_queue_example_1() {
        let mut queue = MyQueue::new();
        assert!(queue.empty());
        queue.push(1);
        queue.push(2);
        assert!(!queue.empty());
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert!(!queue.empty());
        assert_eq!(queue.peek(), 2);
        assert_eq!(queue.pop(), 2);
        assert!(queue.empty());
    }
}
