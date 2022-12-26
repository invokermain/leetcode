// This algorithm iterates the vector in the forward direction, and then for each item
// iterates the remaining right hand side items in reverse. This solution gives us 74ms,
// and beats 16.3% of solutions.
fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for (left_idx, left) in (&numbers).iter().enumerate() {
        for (right_idx, right) in (&numbers[left_idx + 1..]).iter().enumerate().rev() {
            println!("right_idx: {}, right: {}", right_idx, right);
            let sum = left + right;
            if sum > target {
                continue;
            }
            if sum < target {
                break;
            }
            return vec![left_idx as i32 + 1, left_idx as i32 + right_idx as i32 + 2];
        }
    }

    panic!("No solution")
}

// This solution is faster as it is O(N) at most I believe. At most it can only traverse
// the input array once. This solution takes 2ms and beats 84.8% of solutions.
fn two_sum_alt(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left_iter = numbers.iter().enumerate();
    let mut right_iter = numbers.iter().enumerate().rev();

    let mut left_val = left_iter.next().unwrap();
    let mut right_val = right_iter.next().unwrap();
    let mut value = left_val.1 + right_val.1;

    loop {
        if value < target {
            left_val = left_iter.next().unwrap();
        } else if value > target {
            right_val = right_iter.next().unwrap();
        } else {
            return vec![left_val.0 as i32 + 1, right_val.0 as i32 + 1]
        }
        value = left_val.1 + right_val.1;
    }
}

// This solution is faster as it is O(N) at most I believe. At most it can only traverse
// the input array once. This solution takes 2ms and beats 84.8% of solutions.
fn two_sum_alt2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;

    loop {
        let value = numbers[left] + numbers[right];
        if value < target {
            left += 1;
        } else if value > target {
            right -= 1;
        } else {
            return vec![left as i32 + 1, right as i32 + 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(two_sum_alt(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(two_sum_alt2(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(two_sum_alt(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(two_sum_alt2(vec![2, 3, 4], 6), vec![1, 3]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(two_sum(vec![-1, 0], -1), vec![1, 2]);
        assert_eq!(two_sum_alt(vec![-1, 0], -1), vec![1, 2]);
        assert_eq!(two_sum_alt2(vec![-1, 0], -1), vec![1, 2]);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(two_sum(vec![1, 3, 5, 7, 11, 13], 24), vec![5, 6]);
        assert_eq!(two_sum_alt(vec![1, 3, 5, 7, 11, 13], 24), vec![5, 6]);
        assert_eq!(two_sum_alt2(vec![1, 3, 5, 7, 11, 13], 24), vec![5, 6]);
    }
}
