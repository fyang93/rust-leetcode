// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
          next: None,
          val
        }
    }
}

pub fn num_to_list(num: i32) -> Option<Box<ListNode>> {
    let mut num = num;
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut curr = &mut head;

    while num > 0 {
        if let Some(n) = curr {
            n.next.get_or_insert(Box::new(ListNode::new(num % 10)));
            curr = &mut n.next;
            num /= 10;
        }
    }

    head.unwrap().next
}

pub fn list_to_num(list: Option<Box<ListNode>>) -> i32 {
    let mut list = list;
    let mut curr = &mut list;
    let mut num = 0;
    let mut scale = 1;
    while let Some(n) = curr {
        num += n.val * scale;
        curr = &mut n.next;
        scale *= 10;
    }
    num
}
