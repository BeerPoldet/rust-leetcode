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
  pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
  ) -> bool {
    dbg!(&root, &sub_root);
    let Some(sub_root) = sub_root else {
      return true;
    };
    let Some(root) = root else {
      return false;
    };

    Self::is_same(Some(root.clone()), Some(sub_root.clone()))
      || Self::is_subtree(root.borrow().left.clone(), Some(sub_root.clone()))
      || Self::is_subtree(root.borrow().right.clone(), Some(sub_root))
  }

  pub fn is_same(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
    dbg!(&a, &b);
    let (Some(a), Some(b)) = (a.as_ref(), b.as_ref()) else {
      return a.is_none() && b.is_none();
    };
    let a = a.borrow();
    let b = b.borrow();

    a.val == b.val
      && Self::is_same(a.left.clone(), b.left.clone())
      && Self::is_same(a.right.clone(), b.right.clone())
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
    assert!(Solution::is_subtree(
      Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
          val: 4,
          left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
          right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
      }))),
      Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
      })))
    ),);
  }

  #[test]
  fn ex2() {
    assert!(!Solution::is_subtree(
      Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
          val: 4,
          left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
          right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: None
          }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
      }))),
      Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
      })))
    ),);
  }

  #[test]
  fn ex3() {
    assert!(Solution::is_subtree(
      Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: None
      }))),
      Some(Rc::new(RefCell::new(TreeNode::new(1))))
    ),);
  }
}
