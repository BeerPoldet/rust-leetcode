struct Solution;

impl Solution {
  pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn backtrack(
      open_n: i32,
      close_n: i32,
      n: i32,
      result: &mut Vec<String>,
      stack: &mut Vec<String>,
    ) {
      if open_n == n && close_n == n {
        result.push(stack.join(""));
        return;
      }
      if open_n < n {
        stack.push("(".to_string());
        backtrack(open_n + 1, close_n, n, result, stack);
        stack.pop();
      }
      if close_n < open_n {
        stack.push(")".to_string());
        backtrack(open_n, close_n + 1, n, result, stack);
        stack.pop();
      }
    }

    let mut stack = Vec::<String>::new();
    let mut result = Vec::<String>::new();

    backtrack(0, 0, n, &mut result, &mut stack);
    result
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(
      Solution::generate_parenthesis(3),
      ["((()))", "(()())", "(())()", "()(())", "()()()"]
    );
  }

  #[test]
  fn ex2() {
    assert_eq!(Solution::generate_parenthesis(1), ["()"]);
  }
}
