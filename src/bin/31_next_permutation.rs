fn main() {
    let mut nums = vec![2, 3, 1];
    Solution::next_permutation(&mut nums);
}
// [3, 4, 2, 1]
// [4, 1, 2, 3]

struct Solution;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut right = Vec::<i32>::new();
        let mut cut_idx = -1;
        for (idx, &num) in nums.iter().enumerate().rev() {
            if let Some(&max_right) = right.iter().max() {
                if num < max_right {
                    if let Some(min_right_gt_num) = right
                        .iter_mut()
                        .filter(|&&mut right_num| right_num > num)
                        .min()
                    {
                        nums[idx] = *min_right_gt_num;
                        *min_right_gt_num = num;
                        cut_idx = idx as i8;
                        break;
                    }
                }
            }

            right.push(num);
        }
        right.sort();
        let mut right_iter = right.iter();
        for num in nums.iter_mut().skip((cut_idx + 1) as usize) {
            if let Some(&right_num) = right_iter.next() {
                *num = right_num;
            }
        }
        println!("final: {:?}", nums);
    }
}
