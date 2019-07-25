use crate::treenode::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNodeInfo {
    pub val: i32,
    pub depth: usize,
}
impl TreeNodeInfo {
    #[inline]
    pub fn new(val: i32, depth: usize) -> Self {
        TreeNodeInfo {
            val: val,
            depth: depth,
        }
    }
}

pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
    let mut infos = VecDeque::new();
    parse_infos(&s, &mut infos);
    make_tree(0, &mut infos)
}

fn make_tree(depth: usize, infos: &mut VecDeque<TreeNodeInfo>) -> Option<Rc<RefCell<TreeNode>>> {
    match infos.back().filter(|info| info.depth == depth) {
        Some(_) => Some(Rc::new(RefCell::new(TreeNode {
            val: infos.pop_back().unwrap().val,
            left: make_tree(depth + 1, infos),
            right: make_tree(depth + 1, infos),
        }))),
        None => None,
    }
}

// faster than recursion
pub fn recover_from_preorder_stack(s: String) -> Option<Rc<RefCell<TreeNode>>> {
    // path 记录前序遍历的节点路线
    let mut path: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
    let mut infos = VecDeque::new();
    parse_infos(&s, &mut infos);

    while let Some(info) = infos.pop_back() {
        while path.len() > info.depth {
            path.pop();
        }

        let child = Some(Rc::new(RefCell::new(TreeNode::new(info.val))));
        if let Some(item) = path.last() {
            if let Some(rc) = item {
                let mut parent = rc.borrow_mut();
                if parent.left.is_none() {
                    parent.left = child.clone();
                } else {
                    parent.right = child.clone();
                }
            }
        }
        path.push(child);
    }
    path.remove(0)
}

fn parse_infos(s: &str, infos: &mut VecDeque<TreeNodeInfo>) {
    let start_of_val = if s.starts_with("-") {
        s.find(|c| c != '-').unwrap()
    } else {
        0
    };
    let s = &s[start_of_val..];
    match s.find(|c| c == '-') {
        Some(end_of_val) => {
            infos.push_front(TreeNodeInfo::new(
                s[..end_of_val]
                    .as_bytes()
                    .iter()
                    .fold(0, |val, byte| val * 10 + (byte - b'0') as i32),
                start_of_val,
            ));
            parse_infos(&s[end_of_val..], infos);
        }
        None => infos.push_front(TreeNodeInfo::new(
            s.as_bytes()
                .iter()
                .fold(0, |val, byte| val * 10 + (byte - b'0') as i32),
            start_of_val,
        )),
    };
}
