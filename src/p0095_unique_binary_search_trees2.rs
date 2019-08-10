use crate::treenode::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if n == 0 {
        return vec![];
    }
    generate_sub_trees(1, n)
}

fn generate_sub_trees(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    // end >= start
    match end - start {
        0 => vec![Some(Rc::new(RefCell::new(TreeNode::new(start))))],
        1 => {
            let mut node1 = TreeNode::new(start);
            node1.right = Some(Rc::new(RefCell::new(TreeNode::new(end))));
            let mut node2 = TreeNode::new(end);
            node2.left = Some(Rc::new(RefCell::new(TreeNode::new(start))));
            vec![
                Some(Rc::new(RefCell::new(node1))),
                Some(Rc::new(RefCell::new(node2))),
            ]
        }
        _ => {
            let mut res = vec![];
            res.extend(generate_sub_trees(start + 1, end).into_iter().map(|tree| {
                Some(Rc::new(RefCell::new(TreeNode {
                    val: start,
                    left: None,
                    right: tree,
                })))
            }));
            res.extend(generate_sub_trees(start, end - 1).into_iter().map(|tree| {
                Some(Rc::new(RefCell::new(TreeNode {
                    val: end,
                    left: tree,
                    right: None,
                })))
            }));
            for i in start + 1..end {
                for left in generate_sub_trees(start, i - 1) {
                    for right in generate_sub_trees(i + 1, end) {
                        let mut root = TreeNode::new(i);
                        root.left = left.clone();
                        root.right = right;
                        res.push(Some(Rc::new(RefCell::new(root))));
                    }
                }
            }
            res
        }
    }
}
