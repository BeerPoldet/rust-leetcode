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
  pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let Some(root) = root else {
      return vec![];
    };

    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::from([root]);
    let mut result: Vec<i32> = vec![];
    while !queue.is_empty() {
      let mut num: Option<i32> = None;
      for _ in 0..queue.len() {
        let Some(node) = queue.pop_front() else {
          continue;
        };
        num = Some(node.borrow().val);
        if let Some(left) = node.borrow().left.clone() {
          queue.push_back(left);
        }
        if let Some(right) = node.borrow().right.clone() {
          queue.push_back(right);
        };
      }
      if let Some(num) = num {
        result.push(num);
      };
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
      Solution::right_side_view(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
          val: 2,
          left: None,
          right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
          val: 3,
          left: None,
          right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
      }))),),
      vec![1, 3, 4]
    );
  }

  #[test]
  fn ex2() {
    assert_eq!(
      Solution::right_side_view(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
          val: 2,
          left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: None,
          }))),
          right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
      }))),),
      vec![1, 3, 4, 5]
    );
  }
}
