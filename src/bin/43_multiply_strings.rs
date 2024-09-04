use std::char::from_digit;

fn main() {
    let num2 = "456".to_string();
    let num1 = "123".to_string();
    let result = Solution::multiply(num1, num2);
    println!("result: {:?}", result);
}

struct Solution;
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string()
        }
        let mut products = vec![];
        let mut carry = 0;
        let mut i = 0;
        let mut num1_iter = num1.chars().rev();
        while let Some(x) = num1_iter.next() {
            let mut num2_iter = num2.chars().rev().peekable();
            let mut product = vec![0; i];
            let x = x.to_digit(10).unwrap();
            while let Some(y) = num2_iter.next() {
                let y = y.to_digit(10).unwrap();
                let p = x * y + carry;
                let tens_digit = p / 10;
                let ones_digit = p % 10;
                product.push(ones_digit);
                carry = tens_digit;

                if num2_iter.peek().is_none() && carry > 0 {
                    product.push(carry);
                    carry = 0;
                }
            }
            products.push(product);
            i += 1;
        }

        println!("products: {:?}", products);
        let mut result = String::new();
        i = 0;
        carry = 0;
        let mut sum = 0;
        let max_len = products
            .iter()
            .map(|product| product.len())
            .max()
            .unwrap_or(0);
        while i < max_len {
            sum = products
                .iter()
                .map(|product| if i < product.len() { product[i] } else { 0 })
                .sum::<u32>()
                + carry;
            let ones_digit = sum % 10;
            let tens_digit = sum / 10;
            carry = tens_digit;
            result.push(from_digit(ones_digit, 10).unwrap());
            if i == max_len - 1 && carry > 0{
                result.push(from_digit(carry, 10).unwrap())
            }
            i += 1;
        }

        result.chars().rev().collect()
    }
}
