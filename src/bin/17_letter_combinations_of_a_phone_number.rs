use std::collections::HashMap;

fn main() {
    let digits = "999".to_string();
    Solution::letter_combinations(digits);
}

struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![]
        }
        let mut num_to_letters = HashMap::new();
        num_to_letters.insert('2', "abc");
        num_to_letters.insert('3', "def");
        num_to_letters.insert('4', "ghi");
        num_to_letters.insert('5', "jkl");
        num_to_letters.insert('6', "mno");
        num_to_letters.insert('7', "pqrs");
        num_to_letters.insert('8', "tuv");
        num_to_letters.insert('9', "wxyz");
        let first_char = digits.chars().next().unwrap();
        let mut result: Vec<_> = num_to_letters
            .get(&first_char)
            .unwrap()
            .chars()
            .map(|char| char.to_string())
            .collect();
        for digit in digits[1..].chars() {
            let letters = num_to_letters.get(&digit).unwrap();
            let length = letters.len();
            let prev_length = result.len();
            result = result
                .iter()
                .flat_map(|letter| {
                    vec![letter.clone(); length]
                })
                .collect();
            let mut idx = 0usize;
            for char in letters.chars().cycle().take(prev_length * length) {
                result[idx].push(char);
                idx += 1;
            }
        }
        println!("result: {:?}", result);
        result
    }
}
