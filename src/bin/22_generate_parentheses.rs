use std::collections::HashSet;

fn main() {
    let n = 4;
    let result = Solution::generate_parenthesis(n);
    println!("result: {:?}", result);
}

struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 1 {
            return vec!["()".to_string()];
        }

        let mut result = Box::new(HashSet::<String>::new());
        result.insert("()".to_string());
        let mut depth = 1;
        while n != depth {
            result = Self::insert_parenthesis(depth, result);
            depth += 1;
        }

        result.into_iter().collect()
    }

    fn insert_parenthesis(length: i32, result: Box<HashSet<String>>) -> Box<HashSet<String>> {
        let mut parentheses = HashSet::new();
        for parenthesis in result.iter() {
            for idx in 0..length * 2 {
                let mut parenthesis = parenthesis.clone();
                parenthesis.insert_str(idx as usize, "()");
                parentheses.insert(parenthesis);
            }
        }
        Box::new(parentheses)
    }
}
