struct Solution;

impl Solution {
  pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<(usize, i32)> = vec![];
    let mut result: Vec<i32> = vec![0; temperatures.len()];
    for (i, temp) in temperatures.into_iter().enumerate() {
      while let Some(&(top_i, top_t)) = stack.last() {
        if top_t >= temp { break; }
        result[top_i] = (i - top_i) as i32;
        stack.pop();
      }
      stack.push((i, temp));
    }

    result
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(
      Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
      vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
  }

  #[test]
  fn ex2() {
    assert_eq!(
      Solution::daily_temperatures(vec![30, 40, 50, 60]),
      vec![1, 1, 1, 0]
    );
  }

  #[test]
  fn ex3() {
    assert_eq!(
      Solution::daily_temperatures(vec![30, 60, 90]),
      vec![1, 1, 0]
    );
  }
}
