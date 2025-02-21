use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

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

struct FindElements {
    set: std::collections::HashSet<i32>,
}

impl FindElements {
    fn new(mut root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut set = HashSet::new();
        root.as_mut().expect("can not be None").borrow_mut().val = 0;
        set.insert(0);
        Self::traverse_child(root.expect("can not be None"), &mut set);
        FindElements { set }
    }

    fn traverse_child(child: Rc<RefCell<TreeNode>>, set: &mut HashSet<i32>) {
        let node = child.borrow_mut();
        let val = node.val;

        if let Some(left) = node.left.as_ref() {
            let new_val = 2 * val + 1;
            left.borrow_mut().val = new_val;
            set.insert(new_val);
            Self::traverse_child(Rc::clone(left), set);
        }

        if let Some(right) = node.right.as_ref() {
            let new_val = 2 * val + 2;
            right.borrow_mut().val = new_val;
            set.insert(new_val);
            Self::traverse_child(Rc::clone(right), set);
        }
    }

    fn find(&self, target: i32) -> bool {
        self.set.contains(&(target))
    }
}

fn main() {
    let test1_root = Some(Rc::new(RefCell::new(TreeNode {
        val: -1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
    })));
    let test1 = FindElements::new(test1_root);
    assert!(test1.find(2));
    assert!(!test1.find(1));

    let test2_root = Some(Rc::new(RefCell::new(TreeNode {
        val: -1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
    })));
    let test2 = FindElements::new(test2_root);
    assert!(test2.find(1));
    assert!(test2.find(3));
    assert!(!test2.find(5));
    println!("All passed!");
}
