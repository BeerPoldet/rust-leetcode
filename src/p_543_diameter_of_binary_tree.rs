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
  pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
      let Some(node) = node else {
        return 0;
      };
      let mut node = node.borrow_mut();
      let left = dfs(node.left.take(), result);
      let right = dfs(node.right.take(), result);
      let _ = replace(result, max(*result, left + right));
      max(left, right) + 1
    }
    let mut result = 0;
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
      Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
          val: 2,
          left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
          right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
      })))),
      3,
    );
  }
}
