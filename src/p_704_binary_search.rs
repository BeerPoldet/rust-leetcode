struct Solution;

impl Solution {
  pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0i32;
    let mut right = nums.len() as i32 - 1;
    while left <= right {
      let i = (right - left / 2) + left;
      let num = nums[i as usize];
      match num.cmp(&target) {
        std::cmp::Ordering::Less => left = i + 1,
        std::cmp::Ordering::Greater => right = i - 1,
        std::cmp::Ordering::Equal => return i,
      }
    }
    -1
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
  }

  #[test]
  fn ex2() {
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
  }

  #[test]
  fn ex3() {
    assert_eq!(Solution::search(vec![5], 5), 0);
  }

  #[test]
  fn ex4() {
    assert_eq!(Solution::search(vec![5], -5), -1);
  }
}
