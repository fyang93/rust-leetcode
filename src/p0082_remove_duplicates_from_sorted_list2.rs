use crate::listnode::*;

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() { return head; }

    let mut to_remove = head.as_ref().unwrap().val - 1;
    let mut dummy = Some(Box::new(ListNode { next: head, val: to_remove, }));
    let mut node = &mut dummy.as_mut().unwrap().next;

    loop {
        match node {
            None => return dummy.unwrap().next,
            Some(n) if n.val == to_remove => *node = n.next.take(),
            Some(n) if n.next.is_some() && n.val == n.next.as_ref().unwrap().val => to_remove = n.val,
            Some(n) => {
                node = &mut n.next;
                if let Some(n) = node { to_remove = n.val - 1; }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = num_to_list(12);
        let target = num_to_list(12);
        assert_eq!(delete_duplicates(input), target);
    }

    #[test]
    fn it_works_1() {
        let input = num_to_list(32111);
        let target = num_to_list(32);
        assert_eq!(delete_duplicates(input), target);
    }
}
