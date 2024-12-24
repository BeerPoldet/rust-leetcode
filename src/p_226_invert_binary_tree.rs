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
  pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let root = root?;
    {
      let mut root = root.borrow_mut();
      let mut left = root.left.take();
      root.left = Self::invert_tree(root.right.take());
      root.right = Self::invert_tree(left.take());
    }
    Some(root)
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
    assert_eq!(
      Solution::invert_tree(Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
          val: 2,
          left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
          right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
          val: 7,
          left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
          right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        }))),
      })))),
      Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
          val: 7,
          left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
          right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
          val: 2,
          left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
          right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        })))
      }))),
    );
  }
}
