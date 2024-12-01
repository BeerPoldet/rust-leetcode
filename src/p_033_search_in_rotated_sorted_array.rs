struct Solution;

impl Solution {
  pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len() - 1);
    while left <= right {
      let mid = (right + left) / 2;
      if nums[mid] == target {
        return mid as i32;
      }
      if nums[left] <= nums[mid] {
        if nums[mid] < target || nums[left] > target {
          left = mid + 1;
        } else {
          right = mid - 1;
        }
      } else if nums[mid] > target || target > nums[right] {
        right = mid - 1;
      } else {
        left = mid + 1;
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
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
  }

  #[test]
  fn ex2() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
  }

  #[test]
  fn ex3() {
    assert_eq!(Solution::search(vec![1], 0), -1);
  }
}
