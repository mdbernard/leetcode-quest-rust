struct Solution;

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

}

fn main() {}
