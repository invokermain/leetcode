use std::collections::VecDeque;

pub fn total_steps(nums: Vec<i32>) -> i32 {
    let mut stack: VecDeque<(i32, i32)> = VecDeque::new();
    let mut current_max_steps = 0;

    for val in nums.iter().rev() {
        let mut counter = 0;

        while !stack.is_empty() && val > &stack[0].0 {
            let stack_item = stack.pop_front().unwrap();

            counter = if counter >= stack_item.1 {
                counter + 1
            } else {
                stack_item.1
            };
        }

        stack.push_front((*val, counter));

        if counter > current_max_steps {
            current_max_steps = counter;
        }
    }

    current_max_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(total_steps(vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11]), 3)
    }

    #[test]
    fn test_case_2() {
        assert_eq!(total_steps(vec![4, 5, 7, 7, 13]), 0)
    }

    #[test]
    fn test_case_3() {
        assert_eq!(total_steps(vec![3, 1, 1, 2, 1]), 3)
    }

    #[test]
    fn test_case_4() {
        assert_eq!(total_steps(vec![10, 1, 2, 3, 4, 5, 6, 1, 2, 3]), 6)
    }

    #[test]
    fn test_case_5() {
        assert_eq!(total_steps(vec![5, 11, 7, 8, 11]), 2)
    }

    #[test]
    fn test_case_6() {
        assert_eq!(total_steps(vec![7, 11, 1]), 1)
    }

    #[test]
    fn test_case_7() {
        assert_eq!(total_steps(vec![7, 14, 4, 14, 13, 2, 6, 13]), 3)
    }

    #[test]
    fn test_case_8() {
        assert_eq!(total_steps(vec![3, 2, 1, 2]), 2)
    }

    #[test]
    fn test_case_9() {
        assert_eq!(total_steps(vec![1]), 0)
    }

    #[test]
    fn test_case_10() {
        assert_eq!(total_steps(vec![1, 2]), 0)
    }

    #[test]
    fn test_case_11() {
        assert_eq!(total_steps(vec![2, 1]), 1)
    }

    #[test]
    fn test_case_12() {
        assert_eq!(total_steps(vec![3, 2, 1]), 1)
    }
}
