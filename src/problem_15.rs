use std::cmp::Ordering;

fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();

    let mut triplets = Vec::new();

    for i in 0..=nums.len() - 3 {
        let mut j = i + 1;
        let mut k = nums.len() - 1;

        if i > 0 && nums[i - 1] == nums[i] {
            continue;
        }

        if nums[i] > 0 {
            break;
        }

        while j < k {
            let sum = nums[i] + nums[j] + nums[k];
            match Ord::cmp(&sum, &0) {
                Ordering::Less => j += 1,
                Ordering::Equal => {
                    triplets.push(vec![nums[i], nums[j], nums[k]]);

                    let current_k = nums[k];
                    while nums[k] == current_k && k > j {
                        k -= 1;
                    }
                }
                Ordering::Greater => k -= 1,
            };
        }
    }

    triplets
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::three_sum;

    #[test]
    fn test_case_1() {
        let expected: HashSet<Vec<i32>> = HashSet::from_iter(vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
        assert_eq!(
            HashSet::from_iter(three_sum(vec![-1, 0, 1, 2, -1, -4])),
            expected
        )
    }

    #[test]
    fn test_case_2() {
        assert_eq!(three_sum(vec![0, 0, 0]), vec![[0, 0, 0]])
    }

    #[test]
    fn test_case_3() {
        assert_eq!(three_sum(vec![0, 0, 0, 0]), vec![[0, 0, 0]])
    }

    #[test]
    fn test_case_4() {
        let expected: HashSet<Vec<i32>> =
            HashSet::from_iter(vec![vec![-1, 0, 1], vec![-2, 0, 2], vec![-2, -1, 3]]);
        assert_eq!(
            HashSet::from_iter(three_sum(vec![3, 0, -2, -1, 1, 2])),
            expected
        )
    }

    #[test]
    fn test_case_5() {
        let expected: HashSet<Vec<i32>> = HashSet::from_iter(vec![vec![-2, 0, 2], vec![-2, 1, 1]]);
        assert_eq!(
            HashSet::from_iter(three_sum(vec![-2, 0, 1, 1, 2])),
            expected
        )
    }
}
