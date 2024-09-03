use std::collections::HashSet;

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4];
    Solution::three_sum(nums);
}

struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();
        let mut nums = nums;
        nums.sort();
        println!("sorted: {:?}", nums);
        for (idx, &num) in nums[..nums.len() - 2].iter().enumerate() {
            let mut left = idx + 1;
            let mut right = nums.len() - 1;
            while right > left {
                let sum = num + nums[left] + nums[right];
                if sum > 0 {
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    result.push(vec![num, nums[left], nums[right]]);
                    left += 1;
                }
            }
        }
        println!("befor debup: {:?}", result);
        let result = result
            .into_iter()
            .collect::<HashSet<Vec<i32>>>()
            .into_iter()
            .collect::<Vec<Vec<i32>>>();
        println!("result: {:?}", result);
        result
    }
}
