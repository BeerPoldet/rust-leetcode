struct Solution;

impl Solution {
  pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];
    for token in tokens {
      if let Ok(num) = token.parse::<i32>() {
        stack.push(num);
      } else {
        let Some(right) = stack.pop() else {
          continue;
        };
        let Some(left) = stack.pop() else {
          continue;
        };

        let result = match token.as_str() {
          "+" => left + right,
          "-" => left - right,
          "*" => left * right,
          "/" => left / right,
          _ => 0,
        };
        stack.push(result);
      }
    }
    stack.pop().unwrap_or(0)
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(
      Solution::eval_rpn(
        vec!["2", "1", "+", "3", "*"]
          .into_iter()
          .map(std::string::ToString::to_string)
          .collect()
      ),
      9
    );
  }

  #[test]
  fn ex2() {
    assert_eq!(
      Solution::eval_rpn(
        vec!["4", "13", "5", "/", "+"]
          .into_iter()
          .map(std::string::ToString::to_string)
          .collect()
      ),
      6
    );
  }
}
