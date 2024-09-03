fn main() {
    let dividend = -2147483648;
    let divisor = -2;
    let result = Solution::divide(dividend, divisor);
    println!("result: {result}");
}
struct Solution;
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN {
            if divisor == -1 {
                return i32::MAX;
            }
            if divisor == 1 {
                return i32::MIN;
            }
        }

        if divisor == i32::MIN {
            return if dividend == i32::MIN { 1 } else { 0 };
        }

        let is_pos = (dividend >= 0 && divisor >= 0) || (dividend <= 0 && divisor <= 0);
        let mut relax_flag = false;
        let mut dividend = if dividend == i32::MIN {
            relax_flag = true;
            i32::MAX
        } else {
            dividend.abs()
        };
        let divisor = divisor.abs();
        let mut i = 0;
        while dividend >= divisor {
            let mut temp = divisor;
            let mut multiple = 1;
            while dividend >= (temp << 1) && temp <= i32::MAX / 2 {
                println!("temp: {temp}");
                temp <<= 1;
                multiple <<= 1;
            }
            dividend -= temp;
            i += multiple;
        }

        if relax_flag && dividend + 1 >= divisor {
            i += 1;
        }

        if !is_pos {
            -i
        } else {
            i
        }
    }
}
