use std::collections::HashMap;

struct TimeMap {
  store: HashMap<String, Vec<(String, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
  fn new() -> Self {
    Self {
      store: HashMap::new(),
    }
  }

  fn set(&mut self, key: String, value: String, timestamp: i32) {
    self
      .store
      .entry(key)
      .or_insert(vec![])
      .push((value, timestamp));
  }

  fn get(&self, key: String, timestamp: i32) -> String {
    let Some(list) = self.store.get(&key) else {
      return String::new();
    };

    let mut result = Box::new("");

    let (mut l, mut r) = (0i32, list.len() as i32 - 1);
    while l <= r {
      let m = (r + l) / 2;
      let value = &list[m as usize];
      match value.1.cmp(&timestamp) {
        std::cmp::Ordering::Greater => r = m - 1,
        std::cmp::Ordering::Equal => {
          result = Box::new(value.0.as_str());
          break;
        },
        std::cmp::Ordering::Less => {
          result = Box::new(value.0.as_str());
          l = m + 1;
        }
      }
    }

    result.to_string()
  }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

#[cfg(test)]
mod test {
  use super::TimeMap;

  #[test]
  fn ex1() {
    let mut time_map = TimeMap::new();
    time_map.set("foo".to_string(), "bar".to_string(), 1);
    assert_eq!(time_map.get("foo".to_string(), 1), "bar".to_string());
    assert_eq!(time_map.get("foo".to_string(), 3), "bar".to_string());
    time_map.set("foo".to_string(), "bar2".to_string(), 4);
    assert_eq!(time_map.get("foo".to_string(), 4), "bar2".to_string());
    assert_eq!(time_map.get("foo".to_string(), 5), "bar2".to_string());
  }
}
