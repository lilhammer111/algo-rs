fn main() {
    let candidates = vec![2, 3, 6, 7];
    let target = 7;
    let result = Solution::combination_sum(candidates, target);
    println!("result: {:?}", result);
}

struct Solution;
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let mut output = vec![];
        let mut combination = vec![];
        let mut sum = 0;
        let mut i = 0;
        loop {
            combination.push(candidates[i]);
            sum += candidates[i];
            if sum == target {
                output.push(combination);
                combination = vec![];
            } else if sum < target {



            } else {
                break
            }


        }

        vec![]
    }
}
