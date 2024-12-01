struct Solution;

impl Solution {
  pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut max = 0;
    for n in &piles {
      max = std::cmp::max(max, *n);
    }
    let mut left = 1i32;
    let mut right = max;
    let mut result = max;
    let h = h as i64;
    while left <= right {
      let k = (right + left) / 2;
      let hours = Self::calculate_hours(&piles, k);
      match hours.cmp(&h) {
        std::cmp::Ordering::Greater => {
          left = k + 1;
        }
        std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
          result = std::cmp::min(result, k);
          right = k - 1;
        }
      }
    }

    result
  }

  fn calculate_hours(piles: &Vec<i32>, rate: i32) -> i64 {
    let mut sum = 0i64;
    let rate = rate as f64;
    for n in piles {
      sum += (*n as f64 / rate).ceil() as i64;
    }
    sum
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
  }

  #[test]
  fn ex2() {
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
  }

  #[test]
  fn ex3() {
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
  }

  #[test]
  fn ex4() {
    assert_eq!(
      Solution::min_eating_speed(vec![1_000_000_000], 2),
      500_000_000
    );
  }

  #[test]
  fn ex5() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 18), 2);
  }
  #[test]

  fn ex6() {
    assert_eq!(
      Solution::min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000),
      3
    );
  }
}
