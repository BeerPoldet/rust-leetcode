use crate::linked_list::ListNode;

struct Solution;

impl Solution {
  pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut right = dummy.clone();
    let mut left = dummy.as_mut();
    for _ in 0..=n {
      let Some(node) = right else {
        break;
      };
      right = node.next;
    }

    while let Some(right_node) = right {
      right = right_node.next;
      left = left.unwrap().next.as_mut();
    }

    let next = left.as_mut().unwrap().next.as_mut().unwrap().next.clone();
    left.unwrap().next.clone_from(&next);

    dummy.unwrap().next
  }
}

#[cfg(test)]
mod test {
  use crate::linked_list::List;

  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(
      Solution::remove_nth_from_end(List::from(vec![1, 2, 3, 4, 5]).head, 2),
      List::from(vec![1, 2, 3, 5]).head
    );
  }

  #[test]
  fn ex2() {
    assert_eq!(
      Solution::remove_nth_from_end(List::from(vec![1]).head, 1),
      List::from(vec![]).head
    );
  }

  #[test]
  fn ex3() {
    assert_eq!(
      Solution::remove_nth_from_end(List::from(vec![1, 2]).head, 1),
      List::from(vec![1]).head
    );
  }
}
