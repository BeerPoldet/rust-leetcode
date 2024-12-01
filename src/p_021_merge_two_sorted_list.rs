use crate::linked_list::ListNode;

struct Solution;

impl Solution {
  pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let mut prehead = ListNode::new(0);
    let mut cur = &mut prehead;
    let mut list1 = list1;
    let mut list2 = list2;
    while let (Some(list1_node), Some(list2_node)) = (&list1, &list2) {
      // Use clone on the list
      // if list1_node.val < list2_node.val {
      //   cur.next.clone_from(&list1);
      //   list1 = list1.unwrap().next;
      // } else {
      //   cur.next.clone_from(&list2);
      //   list2 = list2.unwrap().next;
      // }
      // if let Some(ref mut next) = cur.next {
      //   cur = next;
      // }
      //
      //
      // Use clone on cur.next
      // if list1_node.val < list2_node.val {
      //   cur.next = list1;
      //   list1 = cur.next.clone().unwrap().next;
      // } else {
      //   cur.next = list2;
      //   list2 = cur.next.clone().unwrap().next;
      // }
      // cur = cur.next.as_mut().unwrap();
      //
      //
      // Use take
      if list1_node.val < list2_node.val {
        cur.next = list1;
        cur = cur.next.as_mut().unwrap();
        list1 = cur.next.take();
      } else {
        cur.next = list2;
        cur = cur.next.as_mut().unwrap();
        list2 = cur.next.take();
      }
    }
    if list1.is_some() {
      cur.next = list1;
    }
    if list2.is_some() {
      cur.next = list2;
    }

    prehead.next
  }
}

#[cfg(test)]
mod test {
  use crate::linked_list::List;

  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(
      Solution::merge_two_lists(
        List::from(vec![1, 2, 4]).head,
        List::from(vec![1, 3, 4]).head,
      ),
      List::from(vec![1, 1, 2, 3, 4, 4]).head,
    );
  }

  #[test]
  fn ex2() {
    assert_eq!(
      Solution::merge_two_lists(List::from(vec![]).head, List::from(vec![]).head,),
      List::from(vec![]).head,
    );
  }

  #[test]
  fn ex3() {
    assert_eq!(
      Solution::merge_two_lists(List::from(vec![]).head, List::from(vec![0]).head,),
      List::from(vec![0]).head,
    );
  }
}

// pub fn merge_two_lists(
//   list1: Option<Box<ListNode>>,
//   list2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
//   let mut list1 = list1;
//   let mut list2 = list2;
//   let mut prehead = ListNode::new(0);
//   let mut cur = &mut prehead;
//   while let (Some(ref mut list1_node), Some(ref mut list2_node)) = (&mut list1, &mut list2) {
//     if list1_node.val < list2_node.val {
//       cur.next = list1.take();
//       cur = cur.next.as_mut().unwrap();
//       list1 = cur.next.take();
//     } else {
//       cur.next = list2.take();
//       cur = cur.next.as_mut().unwrap();
//       list2 = cur.next.take();
//     }
//   }
//   if list1.is_some() {
//     cur.next = list1.take();
//   } else if list2.is_some() {
//     cur.next = list2.take();
//   }
//   prehead.next
// }
