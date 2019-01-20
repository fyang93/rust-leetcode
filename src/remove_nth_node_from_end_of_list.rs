use crate::listnode::*;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut len = 0;

    {
        let mut p = &head;
        while let Some(node) = p {
            len += 1;
            p = &node.next;
        }
    }

    let mut head = head;
    let mut curr = &mut head;
    for _ in 0..len - n {
        curr = &mut curr.as_mut().unwrap().next;
    }

    *curr = curr.as_mut().unwrap().next.take();
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = num_to_list(54321);
        let res = remove_nth_from_end(list, 2);
        let ans = list_to_num(res);
        assert_eq!(ans, 5321);
    }
}
