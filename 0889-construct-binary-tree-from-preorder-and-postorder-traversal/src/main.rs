use std::cell::RefCell;
use std::rc::Rc;

// Solution 1
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

fn construct_tree(
    pre_start: usize,
    pre_end: usize,
    post_start: usize,
    preorder: &Vec<i32>,
    postorder_idx: &Vec<usize>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if pre_start > pre_end {
        return None;
    }
    if pre_start == pre_end {
        return Some(Rc::new(RefCell::new(TreeNode::new(preorder[pre_start]))));
    }

    let left_root = preorder[pre_start + 1];
    let num_nodes_left = postorder_idx[left_root as usize] - post_start + 1;
    let mut root = TreeNode::new(preorder[pre_start]);

    root.left = construct_tree(
        pre_start + 1,
        pre_start + num_nodes_left,
        post_start,
        preorder,
        postorder_idx,
    );
    root.right = construct_tree(
        pre_start + num_nodes_left + 1,
        pre_end,
        post_start + num_nodes_left,
        preorder,
        postorder_idx,
    );

    Some(Rc::new(RefCell::new(root)))
}

pub fn construct_from_pre_post(
    preorder: Vec<i32>,
    postorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut postorder_idx: Vec<usize> = vec![0; preorder.len() + 1];
    for (i, &n) in postorder.iter().enumerate() {
        postorder_idx[n as usize] = i;
    }

    construct_tree(0, preorder.len() - 1, 0, &preorder, &postorder_idx)
}

fn main() {
    println!("All passed!");
}
