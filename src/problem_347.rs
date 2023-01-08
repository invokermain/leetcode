use std::collections::HashMap;

// This algorithm uses a hashmap to count values.
// It is scored at 4ms (75%) and 2.4MB (85%).
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let values_map = nums.iter().fold(HashMap::<i32, u16>::new(), |mut map, x| {
        *map.entry(*x).or_default() += 1;
        map
    });
    let mut values: Vec<(i32, u16)> = values_map.into_iter().collect();
    values.sort_by_key(|x| x.1);
    values.reverse();
    values[..k as usize].iter().map(|x| x.0).collect()
}

#[cfg(test)]
mod tests {
    use super::top_k_frequent;

    #[test]
    fn test_case_1() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2])
    }

    #[test]
    fn test_case_2() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1])
    }

    #[test]
    fn test_case_3() {
        assert_eq!(top_k_frequent(vec![-1, -1], 1), vec![-1])
    }
}
