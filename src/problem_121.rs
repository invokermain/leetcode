use std::cmp::max;

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut buying_point = prices[0];

    for price in &prices[1..] {
        let current_profit = max(price - buying_point, 0);
        max_profit = max(max_profit, current_profit);
        if price < &buying_point {
            buying_point = *price;
        }
    }

    max_profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5)
    }

    #[test]
    fn test_case_2() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0)
    }

    #[test]
    fn test_case_3() {
        assert_eq!(max_profit(vec![2, 3, 1, 4, 3]), 3)
    }

    #[test]
    fn test_case_4() {
        assert_eq!(max_profit(vec![1]), 0)
    }

    #[test]
    fn test_case_5() {
        assert_eq!(max_profit(vec![0, 0, 0]), 0)
    }
}
