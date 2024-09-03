use algo_rs::{create_list, print_list, ListNode};

fn main() {
    let nums = vec![1, 2, 3, 4];
    let list = create_list(nums);
    let result = Solution::swap_pairs(list);
    print_list(&result);
}
struct Solution;
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head;
        let mut nums = vec![];
        while let Some(ref node) = curr {
            nums.push(node.val);
            curr = node.next.clone();
        }

        let mut i = 0;
        while i + 1 < nums.len() {
            nums.swap(i, i + 1);
            i += 2;
        }

        create_list(nums)
    }
}
