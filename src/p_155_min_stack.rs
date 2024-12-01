struct MinStack {
  min: Vec<i32>,
  list: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
  fn new() -> Self {
    Self {
      min: vec![],
      list: vec![],
    }
  }

  fn push(&mut self, val: i32) {
    self.list.push(val);
    if let Some(last) = self.min.last().copied() {
      self.min.push(std::cmp::min(last, val));
    } else {
      self.min.push(val);
    }
  }

  fn pop(&mut self) {
    self.list.pop();
    self.min.pop();
  }

  fn top(&self) -> i32 {
    self.list[self.list.len() - 1]
  }

  fn get_min(&self) -> i32 {
    self.min.last().copied().unwrap_or_default()
  }
}

#[cfg(test)]
mod test {
  use super::MinStack;

  #[test]
  fn ex1() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(min_stack.get_min(), -3);
    min_stack.pop();
    assert_eq!(min_stack.top(), 0);
    assert_eq!(min_stack.get_min(), -2);
  }
}
