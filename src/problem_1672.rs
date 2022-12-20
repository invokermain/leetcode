pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts
        .iter()
        .map(|banks| banks.iter().sum())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::maximum_wealth;

    #[test]
    fn test_case_1() {
        assert_eq!(maximum_wealth(vec![vec![1, 2, 3], vec![1, 2, 3]]), 6)
    }

    #[test]
    fn test_case_2() {
        assert_eq!(maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5], ]), 10)
    }

    #[test]
    fn test_case_3() {
        assert_eq!(maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5], ]), 17)
    }
}
