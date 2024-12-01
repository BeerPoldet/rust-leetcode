struct Solution;

impl Solution {
  pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut prev = vec![1; nums.len()];
    let mut next = vec![1; nums.len()];
    let mut ans = vec![1; nums.len()];
    for i in 1..nums.len() {
      prev[i] = nums[i - 1] * prev[i - 1];
    }
    for i in (0..nums.len() - 1).rev() {
      next[i] = nums[i + 1] * next[i + 1];
    }
    for i in 0..nums.len() {
      ans[i] = prev[i] * next[i];
    }
    ans
  }
}

//  0  1  2  3
// [1, 2, 3, 4]
// [1, 1, 2, 6]
// [24, 12, 4, 1]
// [24, 12, 8, 6]

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(
      Solution::product_except_self(vec![1, 2, 3, 4]),
      vec![24, 12, 8, 6]
    );
  }

  #[test]
  fn ex2() {
    assert_eq!(
      Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
      vec![0, 0, 9, 0, 0]
    );
  }
}
