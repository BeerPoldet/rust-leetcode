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
  pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    let (Some(p), Some(q)) = (p, q) else {
      return None;
    };
    let p = p.borrow();
    let q = q.borrow();
    let mut cur = root;
    while let Some(node) = cur.clone() {
      let node = node.borrow();
      if node.val > p.val && node.val > q.val {
        cur.clone_from(&node.left);
      } else if node.val < p.val && node.val < q.val {
        cur.clone_from(&node.right);
      } else {
        return cur;
      }
    }
    None
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
      Solution::lowest_common_ancestor(
        Some(Rc::new(RefCell::new(TreeNode {
          val: 6,
          left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
              val: 4,
              left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
              right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
          }))),
          right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
          }))),
        }))),
        Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        Some(Rc::new(RefCell::new(TreeNode::new(8)))),
      )
      .unwrap()
      .borrow()
      .val,
      6,
    );
  }

  #[test]
  fn ex2() {
    assert_eq!(
      Solution::lowest_common_ancestor(
        Some(Rc::new(RefCell::new(TreeNode {
          val: 6,
          left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
              val: 4,
              left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
              right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
          }))),
          right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
          }))),
        }))),
        Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        Some(Rc::new(RefCell::new(TreeNode::new(1)))),
      )
      .unwrap()
      .borrow()
      .val,
      2,
    );
  }
  
  #[test]
  fn ex3() {
    assert_eq!(
      Solution::lowest_common_ancestor(
        Some(Rc::new(RefCell::new(TreeNode {
          val: 6,
          left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
              val: 4,
              left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
              right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
          }))),
          right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
          }))),
        }))),
        Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        Some(Rc::new(RefCell::new(TreeNode::new(4)))),
      )
      .unwrap()
      .borrow()
      .val,
      2,
    );
  }
}
