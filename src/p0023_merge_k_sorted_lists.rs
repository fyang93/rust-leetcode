// Tags: LinkedList
use crate::listnode::*;
use std::collections::BinaryHeap;

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap = BinaryHeap::new();

    for list in lists {
        let mut p = &list;
        while let Some(node) = p {
            heap.push(node.val);
            p = &node.next;
        }
    }

    let mut head = None;
    let mut p = &mut head;

    for v in heap.into_sorted_vec() {
        p = &mut p.get_or_insert(Box::new(ListNode::new(v))).next;
    }

    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lists = vec![num_to_list(541), num_to_list(431), num_to_list(62)];
        let merged = merge_k_lists(lists);
        let res = list_to_num(merged);
        assert_eq!(res, 65443211);
    }
}
