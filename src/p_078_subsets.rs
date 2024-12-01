struct Solution;

impl Solution {
  pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(i: usize, nums: &Vec<i32>, stack: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
      if i == nums.len() {
        result.push(stack.clone());
        return;
      }

      // Don't pick number
      backtrack(i + 1, nums, stack, result);

      // Pick number at i
      stack.push(nums[i]);
      backtrack(i + 1, nums, stack, result);
      stack.pop();
    }
    let mut stack: Vec<i32> = vec![];
    let mut result: Vec<Vec<i32>> = vec![];

    backtrack(0, &nums, &mut stack, &mut result);

    result
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(
      Solution::subsets(vec![1, 2, 3]),
      vec![
        vec![],
        vec![3],
        vec![2],
        vec![2, 3],
        vec![1],
        vec![1, 3],
        vec![1, 2],
        vec![1, 2, 3]
      ]
    );
  }

  #[test]
  fn ex2() {
    assert_eq!(Solution::subsets(vec![0]), vec![vec![], vec![0]]);
  }
}
