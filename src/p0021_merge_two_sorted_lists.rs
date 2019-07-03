// Tags: LinkedList
use crate::listnode::*;

pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut p = &mut head;
    let (mut l1, mut l2) = (l1, l2);

    while l1.is_some() && l2.is_some() {
        let (v1, v2) = (l1.as_ref().unwrap().val, l2.as_ref().unwrap().val);
        if v1 < v2 {
            p = &mut p.get_or_insert(Box::new(ListNode::new(v1))).next;
            l1 = l1.unwrap().next;
        } else {
            p = &mut p.get_or_insert(Box::new(ListNode::new(v2))).next;
            l2 = l2.unwrap().next;
        }
    }

    if l1.is_some() { p.get_or_insert(l1.unwrap()); }
    if l2.is_some() { p.get_or_insert(l2.unwrap()); }

    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let l1 = num_to_list(421);
        let l2 = num_to_list(431);
        let merged = merge_two_lists(l1, l2);
        let res = list_to_num(merged);
        assert_eq!(res, 443211);
    }
}
