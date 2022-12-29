// We could use a hashmap here but no need as we have a fixed size of possible inputs.
// Therefore a fixed size array can be used. This solution is 0ms.
fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_counts: [u16; 26] = [0; 26];
    let mut t_counts: [u16; 26] = [0; 26];

    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    for idx in 0..s.len() {
        s_counts[s_bytes[idx] as usize - 97] += 1;
        t_counts[t_bytes[idx] as usize - 97] += 1;
    }

    s_counts == t_counts
}

#[cfg(test)]
mod tests {
    use super::is_anagram;

    #[test]
    fn test_case_1() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()))
    }

    #[test]
    fn test_case_2() {
        assert!(!is_anagram("rat".to_string(), "car".to_string()))
    }
}
