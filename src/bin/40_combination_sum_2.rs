use std::collections::HashMap;
use std::iter::repeat;

fn main() {
    let candidates = vec![10, 1, 2, 7, 6, 1, 5];
    let target = 8;
    let result = Solution::combination_sum2(candidates, target);
    println!("result: {:?}", result);
}

struct Solution;
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut num_to_count = HashMap::new();
        for num in candidates {
            *num_to_count.entry(num).or_insert(0) += 1;
        }

        let mut nums_counts: Vec<_> = num_to_count.into_iter().collect();
        nums_counts.sort_by_key(|&(num, _)| num);
        let (nums, counts): (Vec<_>, Vec<_>) = nums_counts.into_iter().unzip();

        let mut combination = vec![];
        let mut result = vec![];
        Self::back_trace(target, &mut combination, &mut result, 0, &nums, &counts);
        result
    }

    fn back_trace(
        target: i32,
        combination: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        start: usize,
        nums: &[i32],
        counts: &[i32],
    ) {
        if target == 0 {
            result.push(combination.clone());
            return;
        }

        for i in start..nums.len() {
            let cur_num = nums[i];

            if cur_num > target {
                break;
            }

            for count in 1..=counts[i] {
                if cur_num * count > target {
                    break;
                }

                combination.extend(repeat(cur_num).take(count as usize));
                Self::back_trace(
                    target - cur_num * count,
                    combination,
                    result,
                    i + 1,
                    nums,
                    counts,
                );
                for _ in 0..count {
                    combination.pop();
                }
            }
        }
    }
}
