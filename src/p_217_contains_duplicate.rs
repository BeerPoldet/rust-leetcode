use std::collections::HashSet;

struct Solution;

impl Solution {
  pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::<i32>::new();
    for num in nums {
      if set.contains(&num) {
        return true;
      }
      set.insert(num);
    }
    false
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
  }

  #[test]
  fn ex2() {
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
  }
}
