use crate::listnode::*;

pub fn reverse_between(mut head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
    let mut tail = &mut head;
    for _ in 1..m {
        tail = &mut tail.as_mut().unwrap().next;
    }
    *tail = reverse_k(tail.take(), n - m + 1);
    head
}

fn reverse_k(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut node = &mut head;
    for _ in 0..k {
        match node {
            Some(n) => node = &mut n.next,
            _ => return head,
        }
    }

    let mut rest = node.take();
    while let Some(mut n) = head {
        head = std::mem::replace(&mut n.next, rest);
        rest = Some(n);
    }
    rest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(reverse_between(num_to_list(54321), 2, 4), num_to_list(52341));
    }
}
