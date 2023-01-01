fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    nums.iter()
        .map(|val| {
            sum += val;
            sum
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::running_sum;

    #[test]
    fn test_case_1() {
        assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10])
    }

    #[test]
    fn test_case_2() {
        assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn test_case_3() {
        assert_eq!(running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17])
    }
}
