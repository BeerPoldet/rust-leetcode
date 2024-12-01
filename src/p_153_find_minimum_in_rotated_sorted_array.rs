struct Solution;

impl Solution {
  pub fn find_min(nums: Vec<i32>) -> i32 {
    let (mut left, mut right, mut result) = (0i32, nums.len() as i32 - 1, 5001i32);
    while left <= right {
      let l = nums[left as usize];
      let r = nums[right as usize];
      let mid = (right + left) / 2;
      let m = nums[mid as usize];
      if l <= r {
        result = std::cmp::min(result, l);
        break;
      }
      result = std::cmp::min(result, m);
      if l <= m {
        left = mid + 1;
      } else {
        right = mid - 1;
      }
    }
    result
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
  }

  #[test]
  fn ex2() {
    assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
  }

  #[test]
  fn ex3() {
    assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
  }

  #[test]
  fn ex4() {
    assert_eq!(Solution::find_min(vec![1]), 1);
  }

  #[test]
  fn ex5() {
    assert_eq!(Solution::find_min(vec![2, 1]), 1);
  }

  #[test]
  fn ex6() {
    assert_eq!(Solution::find_min(vec![3, 1, 2]), 1);
  }
}
