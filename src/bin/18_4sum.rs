use std::collections::HashSet;

fn main() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    let result = Solution::four_sum(nums, target);
    println!("result: {:?}", result);
}

struct Solution;
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let target = target as i64;
        let nums_len = nums.len();
        if nums_len < 4 {
            return vec![];
        }
        let mut result = vec![];
        let mut nums = nums;
        nums.sort();
        let slice = &nums[..nums_len - 2];
        for (i, &first_num) in slice.iter().enumerate() {
            let mut j = i + 1;
            for &second_num in &slice[(i + 1)..] {
                let mut left = j + 1;
                let mut right = nums_len - 1;
                while right > left {
                    let sum = first_num as i64
                        + second_num as i64
                        + nums[left] as i64
                        + nums[right] as i64;
                    if sum > target {
                        right -= 1;
                    } else if sum < target {
                        left += 1;
                    } else {
                        result.push(vec![first_num, second_num, nums[left], nums[right]]);
                        left += 1;
                    }
                }
                j += 1;
            }
        }
        result = result
            .into_iter()
            .collect::<HashSet<Vec<i32>>>()
            .into_iter()
            .collect();
        result
    }
}
