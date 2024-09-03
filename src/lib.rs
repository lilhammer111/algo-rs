#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &value in nums.iter().rev() {
        let mut new_node = Box::new(ListNode::new(value));
        new_node.next = head;
        head = Some(new_node);
    }
    head
}

pub fn print_list(mut head: &Option<Box<ListNode>>) {
    while let Some(node) = head {
        print!("{} -> ", node.val);
        head = &node.next;
    }
    println!("None");
}
