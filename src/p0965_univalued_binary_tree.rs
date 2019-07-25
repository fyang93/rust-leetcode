use crate::treenode::*;
use std::rc::Rc;
use std::cell::RefCell;

pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        Some(rc) => {
            let node = rc.borrow();
            if let Some(left) = &node.left {
                if left.borrow().val != node.val {
                    return false;
                }
            }
            if let Some(right) = &node.right {
                if right.borrow().val != node.val {
                    return false;
                }
            }
            is_unival_tree(node.left.clone()) && is_unival_tree(node.right.clone())
        },
        None => true,
    }
}
