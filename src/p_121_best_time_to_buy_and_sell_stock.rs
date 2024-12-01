struct Solution;

impl Solution {
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() <= 1 {
      return 0;
    }

    let (mut left, mut right) = (0usize, 1usize);
    let mut result = 0i32;

    while right < prices.len() {
      if prices[left] > prices[right] {
        left = right;
      } else {
        result = std::cmp::max(result, prices[right] - prices[left]);
      }
      right += 1;
    }
    result
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
  }

  #[test]
  fn ex2() {
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
  }
}
