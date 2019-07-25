use crate::treenode::*;
use std::rc::Rc;
use std::cell::RefCell;

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut vals = vec![];
    Self::traverse(&root, &mut vals);
    vals
}

fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
    if let Some(rc) = root {
        let node = rc.borrow();
        Self::traverse(&node.left, vals);
        Self::traverse(&node.right, vals);
        vals.push(node.val);
    }
}
