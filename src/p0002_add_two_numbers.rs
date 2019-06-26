// Tags: LinkedList
use crate::listnode::*;

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut n1, mut n2) = (l1, l2);
    if n1.is_none() { return n2; }
    if n2.is_none() { return n1; }

    let mut head = Some(Box::new(ListNode::new(0)));
    let mut curr = &mut head;
    let mut res = 0;

    while n1.is_some() || n2.is_some() {
        n1 = n1.and_then(|node| { res += node.val; node.next });
        n2 = n2.and_then(|node| { res += node.val; node.next });

        if let Some(node) = curr {
            node.next.get_or_insert(Box::new(ListNode::new(res % 10)));
            curr = &mut node.next;
        }
        res /= 10;
    }
    if res > 0 {
        curr.as_mut().map(|node| {
            node.next.get_or_insert(Box::new(ListNode::new(1)));
        });
    }
    head.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let l1 = num_to_list(342);
        let l2 = num_to_list(465);
        let sum = add_two_numbers(l1, l2);
        let res = list_to_num(sum);
        assert_eq!(342+465, res);
    }

    #[test]
    fn it_works_1() {
        let l1 = num_to_list(81);
        let l2 = num_to_list(0);
        let sum = add_two_numbers(l1, l2);
        let res = list_to_num(sum);
        assert_eq!(81, res);
    }
}
