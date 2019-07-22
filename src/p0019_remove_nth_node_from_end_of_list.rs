use crate::listnode::*;

// unsafe 1-pass
pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut back: *mut _ = &mut head;
    let mut front = &head;

    for _ in 0..n {
        front = &front.as_ref().unwrap().next;
    }

    unsafe {
        while (*front).is_some() {
            back = &mut (*back).as_mut().unwrap().next;
            front = &front.as_ref().unwrap().next;
        }
        *back = (*back).as_mut().unwrap().next.take();
    }

    head
}

// safe 2-pass
pub fn remove_nth_from_end_safe(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut len = 0;
    {
        let mut node = &head;
        while let Some(n) = node {
            len += 1;
            node = &n.next;
        }
    }

    let mut head = head;
    let mut node = &mut head;
    for _ in 0..len - n {
        node = &mut node.as_mut().unwrap().next;
    }

    *node = node.as_mut().unwrap().next.take();
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(list_to_num(remove_nth_from_end(num_to_list(54321), 2)), 5321);
        assert_eq!(list_to_num(remove_nth_from_end_safe(num_to_list(54321), 2)), 5321);
    }
}
