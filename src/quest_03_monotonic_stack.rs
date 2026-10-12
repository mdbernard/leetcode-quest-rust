use std::iter::zip;

pub struct Solution;

pub struct LargestRectangleArea {
    min_height: i32,
    width: i32,
}

impl LargestRectangleArea {
    pub fn area(&self) -> i32 {
        self.min_height * self.width
    }
}

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = prices.clone();
        let mut stack: Vec<(usize, i32)> = vec![];

        for (i, price) in prices.iter().enumerate() {
            while let Some((last_i, last_price)) = stack.last() {
                if *price <= *last_price {
                    ans[*last_i] = last_price - price;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push((i, *price));
        }

        return ans;
    }

    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0; temperatures.len()];
        let mut stack: Vec<(usize, i32)> = vec![];

        for (i, temp) in temperatures.iter().enumerate() {
            while let Some((last_i, last_temp)) = stack.last() {
                if temp > last_temp {
                    ans[*last_i] = (i - *last_i).try_into().unwrap();
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push((i, *temp));
        }

        return ans;
    }

    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut prev_lower_height_i: Vec<Option<usize>> = vec![None; heights.len()];
        let mut inc_stack: Vec<(usize, i32)> = vec![];
        for (rev_i, height) in heights.iter().rev().enumerate() {
            let i = heights.len() - rev_i - 1;
            while let Some((last_i, last_height)) = inc_stack.last() {
                if height < last_height {
                    prev_lower_height_i[*last_i] = Some(i);
                    inc_stack.pop();
                } else {
                    break;
                }
            }
            inc_stack.push((i, *height))
        }

        let mut next_lower_height_i: Vec<Option<usize>> = vec![None; heights.len()];
        let mut dec_stack: Vec<(usize, i32)> = vec![];
        for (i, height) in heights.iter().enumerate() {
            while let Some((last_i, last_height)) = dec_stack.last() {
                if height < last_height {
                    next_lower_height_i[*last_i] = Some(i);
                    dec_stack.pop();
                } else {
                    break;
                }
            }
            dec_stack.push((i, *height));
        }

        let mut max_area: i32 = 0;
        for (prev_lower_i, next_lower_i, height) in
            zip(zip(&prev_lower_height_i, &next_lower_height_i), &heights)
                .map(|((prev_i, next_i), h)| (prev_i, next_i, h))
        {
            let left_i = if prev_lower_i.is_some() {
                prev_lower_i.unwrap() + 1
            } else {
                0
            };
            let right_i = if next_lower_i.is_some() {
                next_lower_i.unwrap()
            } else {
                heights.len()
            };
            let width = (right_i - left_i) as i32;
            max_area = std::cmp::max(max_area, (width) * height);
        }
        return max_area;
    }

    pub fn largest_rectangle_area_2(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::with_capacity(heights.len());
        let mut max_area: i32 = 0;

        for i in 0..=heights.len() {
            let h = if i == heights.len() { 0 } else { heights[i] };
            while let Some(&top) = stack.last() {
                if heights[top] < h {
                    break;
                }
                stack.pop();
                let left = stack.last().copied().map(|j| j as i32).unwrap_or(-1);
                let width = i as i32 - left - 1;
                max_area = max_area.max(heights[top] * width);
            }
            if i < heights.len() {
                stack.push(i);
            }
        }

        return max_area;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_prices_example_1() {
        assert_eq!(
            Solution::final_prices(vec![8, 4, 6, 2, 3]),
            vec![4, 2, 4, 2, 3]
        )
    }

    #[test]
    fn final_prices_example_2() {
        assert_eq!(
            Solution::final_prices(vec![1, 2, 3, 4, 5]),
            vec![1, 2, 3, 4, 5]
        )
    }

    #[test]
    fn final_prices_example_3() {
        assert_eq!(Solution::final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6])
    }

    #[test]
    fn daily_temperatures_example_1() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        )
    }

    #[test]
    fn daily_temperatures_example_2() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        )
    }

    #[test]
    fn daily_temperatures_example_3() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        )
    }

    #[test]
    fn largest_rectangle_area_example_1() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10)
    }

    #[test]
    fn largest_rectangle_area_example_2() {
        assert_eq!(Solution::largest_rectangle_area_2(vec![2, 4]), 4)
    }

    #[test]
    fn largest_rectangle_area_example_3() {
        assert_eq!(Solution::largest_rectangle_area_2(vec![1]), 1);
    }
}
