use std::collections::HashMap;

// See problem_242.rs for some context. This is just a simple extension of that.
// This algorithm is judged at 7ms (95.77%) and 5.3MB (40%).
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups = HashMap::new();

    for str in strs {
        let mut key_array = [0u16; 26];
        for val in str.as_bytes() {
            key_array[*val as usize - 97] += 1;
        }

        groups.entry(key_array).or_insert(vec![]).push(str);
    }

    groups.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::group_anagrams;

    #[test]
    fn test_case_1() {
        let mut grouped = group_anagrams(
            vec!["eat", "tea", "tan", "ate", "nat", "bat"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        );

        // sort here to ensure consistent ordering when asserting equality below
        for group in grouped.iter_mut() {
            group.sort();
        }
        grouped.sort_by_key(|a| a.len());

        assert_eq!(
            grouped,
            vec![
                vec!["bat"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
                vec!["nat", "tan"].iter().map(|x| x.to_string()).collect(),
                vec!["ate", "eat", "tea"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
            ]
        )
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            group_anagrams(vec!["".to_string()],),
            vec![vec!["".to_string()]]
        )
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            group_anagrams(vec!["a".to_string()],),
            vec![vec!["a".to_string()]]
        )
    }
}
