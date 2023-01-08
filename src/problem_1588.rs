use std::cmp::min;

// This is my original solution, it runs in 0ms.
fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;

    let subarray_caps: Vec<i32> = (0..((arr.len() as i32 + 1) / 2))
        .map(|x| 2 * x + 1)
        .map(|x| min(x, arr.len() as i32 + 1 - x))
        .collect();
    let midpoint = arr.len() / 2;

    for (idx, val) in arr.iter().enumerate() {
        let k = if idx < midpoint {
            idx + 1
        } else {
            arr.len() - idx
        } as i32;

        for subarray_cap in subarray_caps.iter() {
            sum += min(k, *subarray_cap) * val
        }
    }

    sum
}

// This is the optimal solution, explained here: https://www.youtube.com/watch?v=J5IIH35EBVE
// For each item in the array, the number of subarrays that include that item is
// the combination of the number of arrays that start and end at that index.
// The number that start at that index is (len_arr - idx).
// The number that end at that index is (idx + 1).
// Therefore the combination of the two is (len_arr - idx) * (idx + 1).
// To get the number of odd length arrays we can do ((len_arr - idx) * (idx + 1) + 1) // 2.
fn sum_odd_length_subarrays_optimal(arr: Vec<i32>) -> i32 {
    let len_arr = arr.len();
    let mut sum: i32 = 0;

    for (idx, val) in arr.iter().enumerate() {
        sum += val * (((idx + 1) * (len_arr - idx) + 1) / 2) as i32
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let left = vec![1, 4, 2, 5, 3];
        let right = 58;
        assert_eq!(sum_odd_length_subarrays(left.clone()), right);
        assert_eq!(sum_odd_length_subarrays_optimal(left), right);
    }

    #[test]
    fn test_case_2() {
        let left = vec![1, 2];
        let right = 3;
        assert_eq!(sum_odd_length_subarrays(left.clone()), right);
        assert_eq!(sum_odd_length_subarrays_optimal(left), right);
    }

    #[test]
    fn test_case_3() {
        let left = vec![1, 2, 3];
        let right = 12;
        assert_eq!(sum_odd_length_subarrays(left.clone()), right);
        assert_eq!(sum_odd_length_subarrays_optimal(left), right);
    }

    #[test]
    fn test_case_4() {
        let left = vec![1, 2, 3, 4];
        let right = 25;
        assert_eq!(sum_odd_length_subarrays(left.clone()), right);
        assert_eq!(sum_odd_length_subarrays_optimal(left), right);
    }

    #[test]
    fn test_case_5() {
        let left = vec![10, 11, 12];
        let right = 66;
        assert_eq!(sum_odd_length_subarrays(left.clone()), right);
        assert_eq!(sum_odd_length_subarrays_optimal(left), right);
    }

    #[test]
    fn test_case_6() {
        let left = vec![6, 9, 14, 5, 3, 8, 7, 12, 13, 1];
        let right = 878;
        assert_eq!(sum_odd_length_subarrays(left.clone()), right);
        assert_eq!(sum_odd_length_subarrays_optimal(left), right);
    }
}
