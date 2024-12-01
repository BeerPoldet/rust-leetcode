use std::collections::HashMap;

struct Solution;

impl Solution {
  pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut count = HashMap::<char, i32>::new();
    let mut result = 0;
    let mut l = 0;
    let s = s.chars().collect::<Vec<_>>();
    let mut maxf = 0;
    for r in 0..s.len() {
      count
        .entry(s[r])
        .and_modify(|c| *c += 1)
        .or_insert(1);
      maxf = std::cmp::max(
        maxf,
        *count.get(&s[r]).unwrap_or(&0)
      );
      let r = r as i32;
      let mut window = r - l + 1;
      while window - maxf > k {
        count.entry(s[l as usize]).and_modify(|c| *c -= 1);
        l += 1;
        window = r - l + 1;
      }
      result = std::cmp::max(result, window);
    }
    result
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
  }

  #[test]
  fn ex2() {
    assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
  }
}
