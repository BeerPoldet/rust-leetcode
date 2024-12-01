struct Solution;

impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let len = s.len();
    if len == 0 {
      return 0;
    }
    if len == 1 {
      return 1;
    }
    let mut cache = std::collections::HashSet::<char>::new();
    let mut longest = 1i32;
    let mut left = 0usize;
    for right in 0..len {
      let r_char = s[right];
      let mut l_char = s[left];
      while left < right && cache.contains(&r_char) {
        cache.remove(&l_char);
        left += 1;
        l_char = s[left];
      }
      cache.insert(r_char);
      longest = std::cmp::max(right as i32 - left as i32 + 1, longest);
    }
    longest
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(
      Solution::length_of_longest_substring("abcabcbb".to_string()),
      3
    );
  }

  #[test]
  fn ex2() {
    assert_eq!(
      Solution::length_of_longest_substring("bbbbb".to_string()),
      1
    );
  }

  #[test]
  fn ex3() {
    assert_eq!(
      Solution::length_of_longest_substring("pwwkew".to_string()),
      3
    );
  }
}
