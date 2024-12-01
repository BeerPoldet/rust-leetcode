struct Solution;

impl Solution {
  pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
      return false;
    }
    let s1 = s1.chars().collect::<Vec<_>>();
    let s2 = s2.chars().collect::<Vec<_>>();

    let mut s1_track = [0; 26];
    let mut s2_track = [0; 26];
    let mut matches = 0;
    for i in 0..s1.len() {
      s1_track[s1[i].to_ascii_lowercase() as usize - 'a'.to_ascii_lowercase() as usize] += 1;
      s2_track[s2[i].to_ascii_lowercase() as usize - 'a'.to_ascii_lowercase() as usize] += 1;
    }
    for i in 0..s1_track.len() {
      if s1_track[i] == s2_track[i] {
        matches += 1;
      }
    }
    for (l, r) in (s1.len()..s2.len()).enumerate() {
      if matches == 26 {
        return true;
      }
      let s = s2[r].to_ascii_lowercase() as usize - 'a'.to_ascii_lowercase() as usize;
      s2_track[s] += 1;
      if s1_track[s] == s2_track[s] {
        matches += 1;
      } else {
        matches -= 1;
      }
      let s = s2[l].to_ascii_lowercase() as usize - 'a'.to_ascii_lowercase() as usize;
      s2_track[s] -= 1;
      if s1_track[s] == s2_track[s] {
        matches += 1;
      } else {
        matches -= 1;
      }
    }
    matches == 26
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(
      Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
      true
    );
  }

  #[test]
  fn ex2() {
    assert_eq!(
      Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
      false
    );
  }

  #[test]
  fn ex3() {
    assert_eq!(
      Solution::check_inclusion("adc".to_string(), "dcda".to_string()),
      true
    );
  }

  #[test]
  fn ex4() {
    assert_eq!(
      Solution::check_inclusion("adc".to_string(), "bbbca".to_string()),
      false
    );
  }
}
