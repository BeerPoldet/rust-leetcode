struct Solution;

impl Solution {
  pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut position_speed: Vec<(i32, i32)> = vec![(0, 0); position.len()];
    for i in 0..position.len() {
      position_speed[i] = (position[i], speed[i]);
    }
    position_speed.sort_by(|a, b| b.0.cmp(&a.0));

    let mut stack: Vec<f32> = Vec::new();
    for (position, speed) in position_speed.into_iter() {
      let time = (target as f32 - position as f32) / speed as f32;
      let Some(&last_time) = stack.last() else {
        stack.push(time);
        continue;
      };
      if last_time < time {
        stack.push(time);
      }
    }

    stack.len() as i32
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(
      Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
      3
    );
  }

  #[test]
  fn ex2() {
    assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
  }

  #[test]
  fn ex3() {
    assert_eq!(Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
  }

  #[test]
  fn ex4() {
    assert_eq!(Solution::car_fleet(10, vec![0, 4, 2], vec![2, 1, 3]), 1);
  }
}
