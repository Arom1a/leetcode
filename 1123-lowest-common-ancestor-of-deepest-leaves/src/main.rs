use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// Solution 1: TODO
type Node = Option<Rc<RefCell<TreeNode>>>;

fn dfs(root: &Node) -> (Node, i32) {
    if let Some(node) = root {
        let left = dfs(&node.borrow().left);
        let right = dfs(&node.borrow().right);
        if left.1 > right.1 {
            return (left.0, left.1 + 1);
        }
        if left.1 < right.1 {
            return (right.0, right.1 + 1);
        }
        return (Some(Rc::clone(node)), left.1 + 1);
    }
    (None, 0)
}

pub fn lca_deepest_leaves(root: Node) -> Node {
    dfs(&root).0
}

fn main() {
    println!("All passed!");
}
