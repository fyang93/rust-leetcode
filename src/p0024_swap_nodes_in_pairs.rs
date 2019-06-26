use crate::listnode::*;
use std::mem;

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;

    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return head;
    }

    let mut rest = swap_pairs(head.as_mut().unwrap().next.as_mut().unwrap().next.take());
    let mut next = mem::replace(&mut head.as_mut().unwrap().next, rest);
    next.as_mut().unwrap().next = head;

    next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = num_to_list(4321);
        let swapt = swap_pairs(list);
        let res = list_to_num(swapt);
        assert_eq!(res, 3412);
    }
}
