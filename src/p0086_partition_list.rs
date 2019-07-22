use crate::listnode::*;

pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut before_head = Some(Box::new(ListNode { next: head, val: 0 }));
    let mut after_head = Some(Box::new(ListNode::new(0)));
    let mut before_node = &mut before_head.as_mut().unwrap().next;
    let mut after_node = &mut after_head.as_mut().unwrap().next;

    loop {
        match before_node {
            Some(n) if n.val >= x => {
                after_node.get_or_insert(Box::new(ListNode::new(n.val)));
                after_node = &mut after_node.as_mut().unwrap().next;
                *before_node = n.next.take();
            },
            Some(n) => before_node = &mut n.next,
            None => {
                *before_node = after_head.as_mut().unwrap().next.take();
                break;
            }
        }
    }

    before_head.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(partition(num_to_list(252341), 3), num_to_list(534221));
    }
}
