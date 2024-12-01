use std::vec;

pub struct List {
  pub head: Option<Box<ListNode>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

impl From<Vec<i32>> for List {
  fn from(vec: Vec<i32>) -> List {
    let mut prehead = ListNode::new(0);
    let mut cur = &mut prehead;
    for val in vec {
      let node = ListNode::new(val);
      cur.next = Some(Box::new(node));
      if let Some(ref mut node) = cur.next {
        cur = node;
      }
    }
    List { head: prehead.next }
  }

  // fn from(list: Vec<i32>) -> List {
  //   let mut head: Option<Box<ListNode>> = None;
  //   let mut cur: &mut Option<Box<ListNode>> = &mut head;
  //   for val in list {
  //     *cur = Some(Box::new(ListNode { val, next: None }));
  //     // cur = cur.next; move cur to point to next empty space
  //     // which will be used for the new node in the next loop.
  //     // cur = &mut cur.as_mut().unwrap().next;
  //     if let Some(ref mut node) = cur {
  //       cur = &mut node.next;
  //     }
  //   }
  //   List { head }
  // }
}

impl From<List> for Vec<i32> {
  fn from(list: List) -> Vec<i32> {
    let mut cur = list.head;
    let mut list = vec![];
    while let Some(node) = cur {
      list.push(node.val);
      cur = node.next;
    }
    list
  }
}

#[cfg(test)]
mod test {
  use crate::linked_list::{List, ListNode};

  #[test]
  fn ex1() {
    assert_eq!(
      List::from(vec![1, 2, 3]).head,
      Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
          val: 2,
          next: Some(Box::new(ListNode { val: 3, next: None }))
        }))
      }))
    );
  }
}
