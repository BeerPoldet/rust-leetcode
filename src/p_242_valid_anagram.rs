use std::collections::HashMap;

struct Solution;

impl Solution {
  pub fn is_anagram(s: String, t: String) -> bool {
    let s_map = s.chars().fold(HashMap::<char, u32>::new(), |mut map, c| {
      *map.entry(c).or_insert(0) += 1;
      map
    });
    let t_map = t.chars().fold(HashMap::<char, u32>::new(), |mut map, c| {
      *map.entry(c).or_insert(0) += 1;
      map
    });
    s_map == t_map
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(
      Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
      true
    );
  }

  #[test]
  fn ex2() {
    assert_eq!(
      Solution::is_anagram("rat".to_string(), "car".to_string()),
      false
    );
  }
}
