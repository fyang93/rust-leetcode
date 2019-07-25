use crate::listnode::*;

pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut node = &mut head;
    for _ in 0..k {
        match node {
            Some(n) => node = &mut n.next,
            _ => return head,
        }
    }

    let mut rest = reverse_k_group(node.take(), k);
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
        assert_eq!(reverse_k_group(num_to_list(54321), 2), num_to_list(53412));
        assert_eq!(reverse_k_group(num_to_list(54321), 3), num_to_list(54123));
    }
}
