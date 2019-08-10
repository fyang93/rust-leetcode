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

pub fn inorder_traversal_stack(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut nums = vec![];
    let mut stack = vec![];
    let mut cur = root.clone();
    while cur.is_some() || !stack.is_empty() {
        while let Some(rc) = cur {
            stack.push(rc.clone());
            cur = rc.borrow().left.clone();
        }
        cur = stack.pop();
        if let Some(rc) = cur {
            nums.push(rc.borrow().val);
            cur = rc.borrow().right.clone();
        }
    }
    nums
}
