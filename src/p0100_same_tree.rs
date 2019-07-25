use crate::treenode::*;
use std::rc::Rc;
use std::cell::RefCell;

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    compare(&p, &q)
}

fn compare(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(rc_p) = p {
        if let Some(rc_q) = q {
            let node_p = rc_p.borrow();
            let node_q = rc_q.borrow();
            if node_p.val != node_q.val {
                return false;
            }
            return compare(&node_p.left, &node_q.left) && compare(&node_p.right, &node_q.right)
        }
        return false;
    }
    q.is_none()
}

// cheat
pub fn is_same_tree_1(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    p == q
}
