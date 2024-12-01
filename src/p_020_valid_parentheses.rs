use std::collections::HashMap;

use crate::solution::Solution;

impl Solution {
  pub fn is_valid(s: String) -> bool {
    let close_to_open = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
      if let Some(open) = close_to_open.get(&c) {
        let Some(last) = stack.pop() else {
          return false;
        };
        if last != *open {
          return false;
        }
      } else {
        stack.push(c);
      }
    }

    stack.is_empty()
  }
}

#[cfg(test)]
mod test {
  use crate::solution::Solution;

  #[test]
  fn ex1() {
    assert_eq!(Solution::is_valid("()".to_string()), true);
  }

  #[test]
  fn ex2() {
    assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
  }

  #[test]
  fn ex3() {
    assert_eq!(Solution::is_valid("(]".to_string()), false);
  }

  #[test]
  fn ex4() {
    assert_eq!(Solution::is_valid("([])".to_string()), true);
  }

  #[test]
  fn ex5() {
    assert_eq!(Solution::is_valid("[".to_string()), false);
  }

  #[test]
  fn ex6() {
    assert_eq!(Solution::is_valid("[]]".to_string()), false);
  }
}
