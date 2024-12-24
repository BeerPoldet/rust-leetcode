use std::cell::RefCell;
use std::cmp::max;
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
  pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let Some(root) = root else {
      return 0;
    };
    let root = root.borrow();
    1 + max(
      Self::max_depth(root.left.clone()),
      Self::max_depth(root.right.clone()),
    )
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
      Solution::max_depth(Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
          val: 20,
          left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
          right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
      })))),
      3,
    );
  }
}
