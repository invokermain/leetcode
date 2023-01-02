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

#[cfg(test)]
mod tests {
    use super::sum_odd_length_subarrays;

    #[test]
    fn test_case_1() {
        assert_eq!(sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58)
    }

    #[test]
    fn test_case_2() {
        assert_eq!(sum_odd_length_subarrays(vec![1, 2]), 3)
    }

    #[test]
    fn test_case_3() {
        assert_eq!(sum_odd_length_subarrays(vec![1, 2, 3]), 12)
    }

    #[test]
    fn test_case_4() {
        assert_eq!(sum_odd_length_subarrays(vec![1, 2, 3, 4]), 25)
    }

    #[test]
    fn test_case_5() {
        assert_eq!(sum_odd_length_subarrays(vec![10, 11, 12]), 66)
    }

    #[test]
    fn test_case_6() {
        assert_eq!(
            sum_odd_length_subarrays(vec![6, 9, 14, 5, 3, 8, 7, 12, 13, 1]),
            878
        )
    }
}
