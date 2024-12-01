use crate::linked_list::ListNode;

struct Solution;

impl Solution {
  // pub fn x() {
  //   let mut x = "hello".to_string();
  //   let mut y = "hello2".to_string();
  //   y = x;
  //   let mut z = y.to_uppercase();
  //   x = "hello3".to_string();
  //   let i = x.to_uppercase();
  //   let mut g = z.to_uppercase();
  //   println!("{g:?}");
  //   // println!("{x:?}");
  // }

  pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head;
    let mut prev: Option<Box<ListNode>> = None;
    while let Some(mut node) = cur {
      // cur = node.next.take();
      // node.next = prev.take();
      // prev = Some(node);
      // let tmp = node.next;
      cur = node.next;
      node.next = prev;
      prev = Some(node);
      // cur = tmp;
    }
    prev
  }
}

#[cfg(test)]
mod test {
  use super::Solution;
  use crate::linked_list::List;

  #[test]
  fn ex1() {
    assert_eq!(
      Solution::reverse_list(List::from(vec![1, 2, 3, 4, 5]).head),
      List::from(vec![5, 4, 3, 2, 1]).head
    );
  }

  // #[test]
  // fn ex2() {
  //   assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
  // }
}
