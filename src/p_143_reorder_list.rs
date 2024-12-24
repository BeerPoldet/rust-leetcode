use crate::linked_list::ListNode;

struct Solution;

impl Solution {
  pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
      return;
    }

    let new_head = head.clone();
    let mut fast = new_head.as_ref();
    let mut slow = head.as_mut();
    while let Some(fast_node) = fast {
      let Some(ref fast_next_node) = &fast_node.next else {
        break;
      };
      slow = slow.unwrap().next.as_mut();
      fast = fast_next_node.next.as_ref();
    }

    let mut prev: Option<Box<ListNode>> = None;
    let mut cur = slow.unwrap().next.take();
    while let Some(mut node) = cur {
      cur = node.next;
      node.next = prev;
      prev = Some(node);
    }

    let mut first = head.as_mut();
    let mut second = prev;
    while let (Some(first_node), Some(mut second_node)) = (first, second) {
      let tmp1 = first_node.next.take();
      let tmp2 = second_node.next.take();

      first_node.next = Some(second_node);
      first_node.next.as_mut().unwrap().next = tmp1;

      first = first_node.next.as_mut().unwrap().next.as_mut();
      second = tmp2;
    }
  }
}

#[cfg(test)]
mod test {
  use crate::linked_list::List;

  use super::Solution;

  #[test]
  fn ex1() {
    let mut list = List::from(vec![1, 2, 3, 4]);
    Solution::reorder_list(&mut list.head);
    assert_eq!(Vec::from(list), vec![1, 4, 2, 3]);
  }

  #[test]
  fn ex2() {
    let mut list = List::from(vec![1, 2, 3, 4, 5]);
    Solution::reorder_list(&mut list.head);
    assert_eq!(Vec::from(list), vec![1, 5, 2, 4, 3]);
  }
}
