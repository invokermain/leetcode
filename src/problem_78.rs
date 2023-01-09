// Given an integer array nums of unique elements, return all possible subsets (the power set).
// The solution set must not contain duplicate subsets. Return the solution in any order.

// This algorithm is actually super simple, we can create a bitmask that has exactly the values
// we need by iterating over the bits in the numbers up to 2^k - 1, where k is the length of
// the nums array. e.g. a length 2 array [1, 2] would create the bitmasks of:
// [0, 0], [0, 1], [1, 0], [1, 1]. Which is just the binary form of the numbers 0 to 3.
fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    Vec::from_iter((0..(2u32.pow(nums.len() as u32))).map(|bitmask| {
        Vec::from_iter(
            (0..nums.len()).filter_map(|idx| match (bitmask >> idx & 1) != 0 {
                true => Some(nums[idx]),
                false => None,
            }),
        )
    }))
}

#[cfg(test)]
mod tests {
    use super::subsets;

    #[test]
    fn test_case_1() {
        let output = subsets(vec![1, 2, 3]);
        let expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];

        assert_eq!(output.len(), expected.len());

        for out in output {
            assert!(expected.contains(&out))
        }
    }

    #[test]
    fn test_case_2() {
        let output = subsets(vec![0]);
        let expected = vec![vec![], vec![0]];

        assert_eq!(output.len(), expected.len());

        for out in output {
            assert!(expected.contains(&out))
        }
    }
}
