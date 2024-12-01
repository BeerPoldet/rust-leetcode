use std::{cell::RefCell, rc::Rc};

struct Solution;

impl Solution {
  fn has_cycle(list: Option<Rc<RefCell<ListNode>>>) -> bool {
    let mut hare = list.clone();
    let mut turtle = list;

    // if let Some(ref mut hare_node) = hare {
    //   let next_hare = hare_node.borrow().next.clone();
    //   if let Some(ref next_hare_node) = next_hare {
    //     let next_hare = next_hare_node.borrow().next.clone();
    //     hare.clone_from(&next_hare);
    //   }
    //   // if let Some(ref next_hare) = hare_node.borrow_mut().next.clone() {
    //     // let next_hare = next_hare.borrow().next.clone();
    //     // let next_hare = hare_node.borrow().next.clone();
    //     // hare.clone_from(&next_hare);
    //   // }
    // }

    // 1.
    // while let Some(hare_node) = hare {
    //   if let Some(next_hare_node) = hare_node.borrow().next.clone() {
    //     hare = next_hare_node.borrow().next.clone();
    //   } else {
    //     break;
    //   }
    //   turtle = turtle.unwrap().borrow().next.clone();
    //
    //   if let (Some(turtle), Some(hare)) = (&turtle, &hare) {
    //     if Rc::ptr_eq(turtle, hare) {
    //       return true;
    //     }
    //   }
    // }

    // 2.
    while let Some(ref hare_node) = hare {
      let next_hare = hare_node.borrow().next.clone();
      if let Some(next_hare_node) = next_hare {
        let next_hare = next_hare_node.borrow().next.clone();
        hare.clone_from(&next_hare);
      } else {
        break;
      }

      if let Some(ref next_turtle_node) = turtle {
        let next_turtle = next_turtle_node.borrow().next.clone();
        turtle.clone_from(&next_turtle);
      }

      if let (Some(turtle), Some(hare)) = (&turtle, &hare) {
        if Rc::ptr_eq(turtle, hare) {
          return true;
        }
      }
    }

    false
  }
}

#[derive(Debug)]
struct ListNode {
  val: i32,
  next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    Self { val, next: None }
  }
}

#[cfg(test)]
mod test {

  use super::{ListNode, Solution};
  use std::{cell::RefCell, rc::Rc};

  #[test]
  fn ex1() {
    let mut node1 = Rc::new(RefCell::new(ListNode::new(3)));
    let mut node2 = Rc::new(RefCell::new(ListNode::new(2)));
    let mut node3 = Rc::new(RefCell::new(ListNode::new(0)));
    let mut node4 = Rc::new(RefCell::new(ListNode::new(-4)));
    node1.borrow_mut().next = Some(node2.clone());
    node2.borrow_mut().next = Some(node3.clone());
    node3.borrow_mut().next = Some(node4.clone());
    node4.borrow_mut().next = Some(node2.clone());
    assert_eq!(Solution::has_cycle(Some(node1)), true);
  }

  #[test]
  fn ex2() {
    // Create nodes without a cycle
    let node1 = Rc::new(RefCell::new(ListNode::new(1)));
    let node2 = Rc::new(RefCell::new(ListNode::new(2)));

    // Link nodes: 1 -> 2
    node1.borrow_mut().next = Some(node2.clone());

    // Test for cycle
    assert!(!Solution::has_cycle(Some(node1)));
  }

  #[test]
  fn ex3() {
    // Empty list
    assert!(!Solution::has_cycle(None));
  }

  #[test]
  fn ex4() {
    let mut node1 = Rc::new(RefCell::new(ListNode::new(3)));
    let mut node2 = Rc::new(RefCell::new(ListNode::new(2)));
    let mut node3 = Rc::new(RefCell::new(ListNode::new(0)));
    let mut node4 = Rc::new(RefCell::new(ListNode::new(-4)));
    node1.borrow_mut().next = Some(node2.clone());
    node2.borrow_mut().next = Some(node3.clone());
    node3.borrow_mut().next = Some(node4.clone());
    assert_eq!(Solution::has_cycle(Some(node1)), false);
  }
}
