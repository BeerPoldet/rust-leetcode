use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

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
      right: None,
    }
  }
}

impl Solution {
  pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let (Some(p), Some(q)) = (p.as_ref(), q.as_ref()) else {
      return p.is_none() && q.is_none();
    };
    let mut p = p.borrow_mut();
    let mut q = q.borrow_mut();
    p.val == q.val
    && Self::is_same_tree(p.left.take(), q.left.take())
    && Self::is_same_tree(p.right.take(), q.right.take())
  }
}

#[cfg(test)]
mod test {
  use super::Solution;
  use super::TreeNode;
  use std::cell::RefCell;
  use std::rc::Rc;

  #[test]
  fn ex1() {
    assert!(Solution::is_same_tree(
      Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
      }))),
      Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
      })))
    ));
  }

  #[test]
  fn ex2() {
    assert!(!Solution::is_same_tree(
      Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: None,
      }))),
      Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
      })))
    ));
  }
}
