fn main() {
    let nums = vec![2, 2];
    let target = 2;
    let result = Solution::search_range(nums, target);
    println!("result: {:?}", result);
}

struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        if nums.len() == 1 && nums[0] == target {
            return vec![0, 0];
        }

        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut result = vec![-1, -1];
        while right >= left {
            let mid = (right + left) / 2;
            let mid_num = nums[mid];
            if target > mid_num {
                left = mid + 1;
            } else if target < mid_num {
                if mid == 0 {
                    break;
                };
                right = mid - 1;
            } else {
                result[0] = mid as i32;
                result[1] = mid as i32;
                let mut i = mid;
                while i != 0 && nums[i - 1] == target {
                    result[0] = (i - 1) as i32;
                    i -= 1;
                }

                i = mid;
                while i != nums.len() - 1 && nums[i + 1] == target {
                    result[1] = (i + 1) as i32;
                    i += 1;
                }
                break;
            }
        }

        result
    }
}
