use crate::treenode::*;
use std::rc::Rc;
use std::cell::RefCell;

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut vals = vec![];
    traverse(&root, &mut vals);
    vals
}

fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
    if let Some(rc) = root {
        let node = rc.borrow();
        vals.push(node.val);
        traverse(&node.left, vals);
        traverse(&node.right, vals);
    }
}

pub fn preorder_traversal_stack(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut vals = vec![];
    if root.is_none() { return vals; }

    let mut stack = vec![root];
    while let Some(item) = stack.pop()  {
        if let Some(rc) = item {
            let node = rc.borrow();
            vals.push(node.val);
            stack.push(node.right.clone());
            stack.push(node.left.clone());
        }
    }
    vals
}
