use std::cmp::min;

fn main() {
    let nums = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 0];
    let result = Solution::jump(nums);
    println!("result: {}", result);
}

struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let n = nums.len();
        let mut count = 0;
        let mut max_val = 0;
        while i != n - 1 {
            count += 1;
            let j = nums[i];
            i += 1;
            if i + j as usize >= n {
                return count;
            }
            let max_distance = min(i + j as usize, n);
            for idx in i..max_distance {
                println!("ele: {}", nums[idx]);
                if nums[idx] >= max_val {
                    i = idx;
                    max_val = nums[idx]
                }
            }
            max_val = 0;
        }
        count
    }
}
