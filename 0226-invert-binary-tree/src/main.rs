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

use std::cell::RefCell;
use std::rc::Rc;
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    root.map(|node| {
        {
            let mut n = node.borrow_mut();
            let l = n.left.take();
            let r = n.right.take();
            n.left = invert_tree(r);
            n.right = invert_tree(l);
        }
        node
    })
}

fn main() {
    println!("Code compiles!");
}
