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

fn parse_input(traversal: String) -> Vec<(i32, usize)> {
    let mut data: Vec<(i32, usize)> = vec![];
    let mut last_num_str: String = String::new();
    let mut dash_num: usize = 0;
    traversal.chars().for_each(|c| {
        if c.is_numeric() {
            last_num_str.push(c);
        }
        if c == '-' {
            if last_num_str != "" {
                data.push((last_num_str.parse().expect("can not be wrong"), dash_num));
                last_num_str = String::new();
                dash_num = 0;
            }
            dash_num += 1;
        }
    });
    data.push((last_num_str.parse().expect("can not be wrong"), dash_num));

    data
}

fn construct_tree(
    data: &Vec<(i32, usize)>,
    depth: usize,
    idx: &mut usize,
) -> Option<Rc<RefCell<TreeNode>>> {
    if *idx == data.len() {
        return None;
    }

    let mut node = TreeNode::new(data[*idx].0);

    if data[*idx].1 != depth {
        return None;
    }

    *idx += 1;
    node.left = construct_tree(data, depth + 1, idx);
    node.right = construct_tree(data, depth + 1, idx);

    Some(Rc::new(RefCell::new(node)))
}

pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
    let data = parse_input(traversal);
    construct_tree(&data, 0, &mut 0)
}

fn main() {
    let test1_result = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(
        recover_from_preorder("1-2--3--4-5--6--7".to_string()),
        test1_result
    );

    let test2_result = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: None,
            }))),
            right: None,
        }))),
    })));
    assert_eq!(
        recover_from_preorder("1-2--3---4-5--6---7".to_string()),
        test2_result
    );

    let test3_result = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 401,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 349,
                left: Some(Rc::new(RefCell::new(TreeNode::new(90)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(88)))),
        }))),
        right: None,
    })));
    assert_eq!(
        recover_from_preorder("1-401--349---90--88".to_string()),
        test3_result
    );
    println!("All passed!");
}
