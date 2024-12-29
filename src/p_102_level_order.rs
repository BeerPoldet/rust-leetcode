use std::cell::RefCell;
use std::collections::VecDeque;
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
  pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let Some(root) = root else {
      return vec![];
    };
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::from([root]);
    let mut result: Vec<Vec<i32>> = Vec::new();
    while !queue.is_empty() {
      let mut level: Vec<i32> = Vec::new();
      for _ in 0..queue.len() {
        let Some(node) = queue.pop_front() else {
          continue;
        };
        level.push(node.borrow().val);
        if let Some(node) = node.borrow().left.clone() {
          queue.push_back(node);
        }
        if let Some(node) = node.borrow().right.clone() {
          queue.push_back(node);
        };
      }
      result.push(level);
    }
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
      Solution::level_order(Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
          val: 20,
          left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
          right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
      }))),),
      vec![vec![3], vec![9, 20], vec![15, 7]]
    );
  }
}
