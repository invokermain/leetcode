use std::collections::HashSet;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::new();

    for item in nums {
        if set.contains(&item) {
            return true;
        }
        set.insert(item);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]))
    }

    #[test]
    fn test_case_2() {
        assert!(!contains_duplicate(vec![1, 2, 3, 4]))
    }

    #[test]
    fn test_case_3() {
        assert!(contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]))
    }
}