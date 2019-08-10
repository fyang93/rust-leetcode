use crate::treenode::*;
use std::rc::Rc;
use std::cell::RefCell;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    bound_check(&root, None, None)
}

fn bound_check(node: &Option<Rc<RefCell<TreeNode>>>, lower: Option<i32>, upper: Option<i32>) -> bool {
    if let Some(rc) = node {
        let val = rc.borrow().val;
        if lower.filter(|&v| v >= val).is_some() || upper.filter(|&v| v <= val).is_some() {
            return false;
        }
        return bound_check(&rc.borrow().left, lower, Some(val)) && bound_check(&rc.borrow().right, Some(val), upper);
    }
    true
}

pub fn is_valid_bst_stack(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut last = None;
    let mut stack = vec![];
    let mut cur = root.clone();
    while cur.is_some() || !stack.is_empty() {
        while let Some(rc) = cur {
            stack.push(rc.clone());
            cur = rc.borrow().left.clone();
        }
        cur = stack.pop();
        if let Some(rc) = cur {
            if last.filter(|&v| v >= rc.borrow().val).is_some() {
                return false;
            }
            last = Some(rc.borrow().val);
            cur = rc.borrow().right.clone();
        }
    }
    true
}
