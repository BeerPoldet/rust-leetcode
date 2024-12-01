use std::collections::HashMap;

use crate::solution::Solution;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::<i32, i32>::new();
    // for (i, n) in nums.into_iter().enumerate() {
    //   if let Some(comp_index) = map.get(&n) {
    //     return vec![*comp_index, i as i32];
    //   }
    //   map.insert(target - n, i as i32);
    // }
    for i in 0..=nums.len() as i32 {
      // let n = nums[i];
      let Some(n) = nums.get(i as usize) else { continue };
      if let Some(comp_index) = map.get(&n) {
        return vec![*comp_index, i];
      }
      map.insert(target - n, i);
    }
    vec![]
  }
}

#[cfg(test)]
mod test {
  use crate::solution::Solution;

  #[test]
  fn ex1() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
  }

  #[test]
  fn ex2() {
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
  }

  #[test]
  fn ex3() {
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
  }
}
