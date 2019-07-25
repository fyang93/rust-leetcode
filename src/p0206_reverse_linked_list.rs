use crate::listnode::*;

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut reversed = None;
    while let Some(mut n) = head {
        head = std::mem::replace(&mut n.next, reversed);
        reversed = Some(n);
    }
    reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(reverse_list(num_to_list(54321)), num_to_list(12345));
    }
}
