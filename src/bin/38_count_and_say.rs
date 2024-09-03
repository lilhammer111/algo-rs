fn main() {
    let n = 10;
    let result = Solution::count_and_say(n);
    println!("result: {:?}", result);
}

struct Solution;
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return String::from("1");
        }
        let prev = Self::count_and_say(n - 1);
        let mut count = 0;
        let mut chars = prev.chars().peekable();
        let mut rle = String::new();
        while let Some(char) = chars.next() {
            count += 1;
            if chars.peek() != Some(&char) {
                rle.push_str(&format!("{}{}", count, char));
                count = 0;
            }
        }
        rle
    }
}
