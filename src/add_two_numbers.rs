// Definition for singly-linked list.
 #[derive(PartialEq, Eq, Debug)]
 pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
 }

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn num_to_list(num: i32) -> Option<Box<ListNode>> {
    let mut num = num;
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut curr = &mut head;

    while num > 0 {
        if let Some(n) = curr {
            n.next.get_or_insert(Box::new(ListNode::new(num % 10)));
            curr = &mut n.next;
            num /= 10;
        }
    }

    head.unwrap().next
}

fn list_to_num(list: Option<Box<ListNode>>) -> i32 {
    let mut list = list;
    let mut curr = &mut list;
    let mut num = 0;
    let mut scale = 1;
    while let Some(n) = curr {
        num += n.val * scale;
        curr = &mut n.next;
        scale *= 10;
    }
    num
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut n1, mut n2) = (l1, l2);
    if n1.is_none() { return n2; }
    if n2.is_none() { return n1; }

    let mut head = Some(Box::new(ListNode::new(0)));
    let mut curr = &mut head;
    let mut res = 0;

    while n1.is_some() || n2.is_some() {
        if let Some(node) = n1 {
            res += node.val;
            n1 = node.next;
        }
        if let Some(node) = n2 {
            res += node.val;
            n2 = node.next;
        }

        if let Some(n) = curr {
            n.next.get_or_insert(Box::new(ListNode::new(res % 10)));
            curr = &mut n.next;
        }
        res /= 10;
    }
    if res > 0 {
        if let Some(n) = curr {
            n.next.get_or_insert(Box::new(ListNode::new(1)));
        }
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
