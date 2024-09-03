fn main() {
    let nums = vec![-1,2,1,-4];
    let target = 1;
    Solution::three_sum_closest(nums, target);
}

struct Solution;
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        println!("sorted: {:?}", nums);
        let length = nums.len();
        let slice = &nums[..length - 2];
        let mut closest_sum = nums.iter().take(3).sum::<i32>();
        println!("init closest_sum: {}", closest_sum);
        for (idx, &num) in slice.iter().enumerate() {
            let mut left = idx + 1;
            let mut right = length - 1;
            while right > left {
                let sum = num + nums[left] + nums[right];
                if (sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = sum;
                }

                if sum > target {
                    right -= 1;
                } else if sum < target {
                    left += 1;
                } else {
                    println!("equal to target");
                    return target;
                }
            }
        }
        println!("final closest sum: {}", closest_sum);
        closest_sum
    }
}
