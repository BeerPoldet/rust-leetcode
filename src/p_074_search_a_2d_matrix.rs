struct Solution;

impl Solution {
  pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let row_count = matrix.len();
    let col_count = matrix[0].len();
    let (mut top, mut bottom) = (0i32, row_count as i32 - 1);
    while top <= bottom {
      let row_idx = (bottom + top) / 2;
      let row = &matrix[row_idx as usize];
      if target > row[col_count - 1] {
        top = row_idx + 1;
      } else if target < row[0] {
        bottom = row_idx - 1;
      } else {
        break;
      }
      if top > bottom {
        return false;
      }
    }

    let row_idx = (bottom + top) / 2;
    let (mut left, mut right) = (0i32, col_count as i32 - 1);
    while left <= right {
      let i = (right + left) / 2;
      let num = matrix[row_idx as usize][i as usize];
      match target.cmp(&num) {
        std::cmp::Ordering::Greater => left = i + 1,
        std::cmp::Ordering::Less => right = i - 1,
        std::cmp::Ordering::Equal => return true,
      }
    }
    false
  }
}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn ex1() {
    assert_eq!(
      Solution::search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        3
      ),
      true
    );
  }

  #[test]
  fn ex2() {
    assert_eq!(
      Solution::search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        13
      ),
      false
    );
  }

  #[test]
  fn ex3() {
    assert_eq!(Solution::search_matrix(vec![vec![1], vec![3]], 13), false);
  }
}
