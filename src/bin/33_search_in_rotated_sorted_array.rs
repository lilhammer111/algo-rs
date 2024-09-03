fn main() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 0;
    let result = Solution::search(nums, target);
    println!("result: {result}")
}

// 洗个澡回来刷题
struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if let Some(idx) = nums.iter().position(|&num| num == target) {
            idx as i32
        } else {
            -1
        }
    }
}
