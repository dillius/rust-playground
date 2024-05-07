use std::cell::RefCell;
use std::rc::Rc;

type TreeCell = Option<Rc<RefCell<TreeNode>>>;

// Definition for a binary tree node.
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
            right: None
        }
    }
}

pub struct Btree;

impl Btree {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::preorder(root)
    }

    fn preorder(root: TreeCell) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                result.push(node.val);
                stack.push(node.right.clone());
                stack.push(node.left.clone());
            }
        }
        result
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::inorder(root)
    }

    fn inorder(root: TreeCell) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![(root, false)];
        while let Some((node, done)) = stack.pop() {
            if done {
                result.push(node.unwrap().borrow().val);
            } else if let Some(n) = node {
                let b = n.borrow();
                stack.push((b.right.clone(), false));
                stack.push((Some(n.clone()), true));
                stack.push((b.left.clone(), false));
            }
        }
        result
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::postorder(root)
    }

    fn postorder(root: TreeCell) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![(root, false)];
        while let Some((node, done)) = stack.pop() {
            if done {
                result.push(node.unwrap().borrow().val);
            } else if let Some(n) = node {
                let b = n.borrow();
                stack.push((Some(n.clone()), true));
                stack.push((b.right.clone(), false));
                stack.push((b.left.clone(), false));
            }
        }
        result
    }
}
