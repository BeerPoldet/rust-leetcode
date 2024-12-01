use std::collections::HashMap;

struct Solution;

impl Solution {
  // Top down
  pub fn fib(n: i32) -> i32 {
    fn fx(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
      if let Some(&n) = cache.get(&n) {
        return n;
      }
      let result = fx(n - 1, cache) + fx(n - 2, cache);
      cache.insert(n, result);
      result
    }
    let mut cache = HashMap::<i32, i32>::from([(0, 0), (1, 1)]);

    fx(n, &mut cache)
  }

  // Bottom up
  // pub fn fib(n: i32) -> i32 {
  //   let mut cache = vec![0; (n + 1) as usize];
  //   if n == 0 || n == 1 { return n; }
  //   cache[0] = 0;
  //   cache[1] = 1;
  //   for i in 2..=n {
  //     let idx = i as usize;
  //     cache[idx] = cache[idx - 1] + cache[idx - 2];
  //   }
  //   cache[n as usize]
  // }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(Solution::fib(2), 1);
  }

  #[test]
  fn ex2() {
    assert_eq!(Solution::fib(3), 2);
  }

  #[test]
  fn ex3() {
    assert_eq!(Solution::fib(4), 3);
  }
}
