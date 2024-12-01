use std::cmp;

use crate::solution::Solution;

impl Solution {
  pub fn max_area(height: &[u64]) -> u64 {
    let (mut left, mut right, mut max) = (0usize, height.len() - 1, 0);
    while left < right {
      let width = (right - left) as u64;
      max = cmp::max(cmp::min(height[left], height[right]) * width, max);
      if height[left] < height[right] {
        left += 1;
      } else {
        right -= 1;
      }
    }
    max
  }
}

#[cfg(test)]
mod test {
  use crate::solution::Solution;

  #[test]
  fn ex1() {
    assert_eq!(Solution::max_area(&[1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
  }

  #[test]
  fn ex2() {
    assert_eq!(Solution::max_area(&[1, 1]), 1);
  }
}
