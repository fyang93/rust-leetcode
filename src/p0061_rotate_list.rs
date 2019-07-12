use crate::listnode::*;

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k == 0 || head.is_none() {
        return head;
    }

    let mut node = &head;
    let mut len = 0;
    while let Some(n) = node {
        len += 1;
        node = &n.next;
    }
    let tail = node;

    let k = k % len;
    if k != 0 {
        node = &head;
        for _ in 0..len - k {
            node = &node.as_ref().unwrap().next;
        }

        unsafe {
            let _tail: *mut Option<Box<ListNode>> = std::mem::transmute(tail);
            let _node: *mut Option<Box<ListNode>> = std::mem::transmute(node);
            let new_head = (*_node).take();
            std::ptr::replace(_tail, head);
            return new_head;
        }
    }
    head
}

pub fn rotate_right_safe(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if  k == 0 || head.is_none() {
        return head;
    }

    let mut node = &head;
    let mut len = 0;
    while let Some(n) = node {
        len += 1;
        node = &n.next;
    }

    let k = k % len;
    if  k == 0 {
        return head;
    }

    if k != 0 {
        let mut head = head;
        let mut node = &mut head;
        for _ in 0..len - k {
            node = &mut node.as_mut().unwrap().next;
        }
        let mut new_head = node.take();
        let mut tail = &mut new_head;
        while let Some(n) = tail {
            tail = &mut n.next;
        }
        tail.get_or_insert(head.unwrap());
        return new_head;
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(rotate_right(num_to_list(54321), 2), num_to_list(32154));
        assert_eq!(rotate_right_safe(num_to_list(54321), 2), num_to_list(32154));
    }
}
