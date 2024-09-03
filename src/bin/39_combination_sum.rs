fn main() {
    let candidates = vec![2, 3, 6, 7];
    let target = 7;
    let result = Solution::combination_sum(candidates, target);
    println!("result: {:?}", result);
}
struct Solution;
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut comb = vec![];
        Self::back_trace(&candidates, target, &mut comb, &mut result, 0);
        result
    }

    fn back_trace(
        candidates: &[i32],
        target: i32,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        start: usize,
    ) {
        if target == 0 {
            result.push(current.clone());
            return;
        }

        for i in start..candidates.len() {
            if target >= candidates[i] {
                current.push(candidates[i]);
                Self::back_trace(candidates, target - candidates[i], current, result, i);
                current.pop();
            }
        }
    }
}
