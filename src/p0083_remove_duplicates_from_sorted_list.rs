use crate::listnode::*;

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() { return head; }
    let mut back: *mut _ = &mut head;
    let mut front: *const _ = &head.as_ref().unwrap().next;

    unsafe {
        while let Some(node) = (*front).as_ref() {
            if (*back).as_ref().unwrap().val != node.val {
                back = &mut (*back).as_mut().unwrap().next;
                (*back).as_mut().unwrap().val = node.val;
            }
            front = &(*front).as_ref().unwrap().next;
        }
        (*back).as_mut().unwrap().next.take();
    }

    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut input = num_to_list(211);
        let target = num_to_list(21);
        assert_eq!(delete_duplicates(input), target);
    }

    #[test]
    fn it_works_1() {
        let mut input = num_to_list(33211);
        let target = num_to_list(321);
        assert_eq!(delete_duplicates(input), target);
    }
}
