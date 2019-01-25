use crate::listnode::*;
use std::mem;

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;

    let mut node = &mut head;
    for _ in 0..k {
        if let Some(n) = node {
            node = &mut n.next;
        } else {
            return head;
        }
    }

    let mut rest = reverse_k_group(node.take(), k);
    let mut curr = head;
    while let Some(mut n) = curr {
        curr = mem::replace(&mut n.next, rest);
        rest = Some(n);
    }

    rest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = num_to_list(54321);
        let swapt = reverse_k_group(list, 2);
        let res = list_to_num(swapt);
        assert_eq!(res, 53412);
    }

    #[test]
    fn it_works_1() {
        let list = num_to_list(54321);
        let swapt = reverse_k_group(list, 3);
        let res = list_to_num(swapt);
        assert_eq!(res, 54123);
    }
}
