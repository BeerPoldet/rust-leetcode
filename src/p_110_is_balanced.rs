use std::cell::RefCell;
use std::cmp::max;
use std::mem::replace;
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
  pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, result: &mut bool) -> i32 {
      let Some(node) = node else {
        return 0;
      };
      let mut node = node.borrow_mut();
      let left = dfs(node.left.take(), result);
      let right = dfs(node.right.take(), result);
      let _ = replace(result, *result && (left - right).abs() <= 1);
      max(left, right) + 1
    }
    let mut result = true;
    dfs(root, &mut result);
    result
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
      Solution::is_balanced(Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
          val: 20,
          left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
          right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
      })))),
      true,
    );
  }

  #[test]
  fn ex2() {
    assert_eq!(
      Solution::is_balanced(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
          val: 2,
          left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
          }))),
          right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
      })))),
      false,
    );
  }
}
