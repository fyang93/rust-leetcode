use crate::treenode::*;
use std::rc::Rc;
use std::cell::RefCell;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut vals = vec![];
    traverse(&root, &mut vals);
    vals
}

fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
    if let Some(rc) = root {
        let node = rc.borrow();
        traverse(&node.left, vals);
        vals.push(node.val);
        traverse(&node.right, vals);
    }
}
