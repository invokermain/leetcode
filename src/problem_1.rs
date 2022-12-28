use std::collections::HashMap;

// This method uses array sorting and is therefore not the most performant
fn two_sum_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums_sorted: Vec<(usize, i32)> = nums.into_iter().enumerate().collect();
    nums_sorted.sort_by_key(|f| f.1);

    for (left_enum_idx, (left_idx, left)) in nums_sorted.iter().enumerate() {
        for (right_idx, right) in &nums_sorted[left_enum_idx + 1..] {
            let sum = left + right;
            if sum > target {
                break;
            }
            if sum < target {
                continue;
            }
            return vec![*left_idx as i32, *right_idx as i32];
        }
    }

    panic!("No solution")
}

// A better solution is to use a Hashmap
fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen_numbers: HashMap<i32, u16> = HashMap::new();

    for (idx, num) in nums.iter().enumerate() {
        if seen_numbers.contains_key(&(target - num)) {
            return vec![seen_numbers[&(target - num)] as i32, idx as i32];
        }
        seen_numbers.insert(*num, idx as u16);
    }

    panic!("No solution")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(two_sum_1(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum_2(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(two_sum_1(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum_2(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(two_sum_1(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(two_sum_2(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(two_sum_1(vec![3, 2, 3], 6), vec![0, 2]);
        assert_eq!(two_sum_2(vec![3, 2, 3], 6), vec![0, 2]);
    }

    #[test]
    fn test_case_5() {
        let vec: Vec<i32> = (1..10001).collect();
        assert_eq!(two_sum_1(vec.clone(), 19999), vec![9998, 9999]);
        assert_eq!(two_sum_2(vec.clone(), 19999), vec![9998, 9999]);
    }
}
