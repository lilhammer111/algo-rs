use algo_rs::{ListNode, print_list};

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let n = 2;
    let mut list_node = Solution::create_list(nums);
    print_list(&list_node);
    list_node = Solution::remove_nth_from_end(list_node, n);
    print_list(&list_node);
}

struct Solution;
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut nums = vec![];
        while head != None {
            if let Some(ref node) = head {
                nums.push(node.val);
                head = node.next.clone();
            }
        }
        println!("nums: {:?}", nums);
        let length = nums.len();
        nums.remove(length - n as usize);
        println!("removed: {:?}", nums);
        Self::create_list(nums)
    }


}

