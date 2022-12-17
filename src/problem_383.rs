fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut letter_counts: [u32; 26] = [0; 26];

    magazine.as_bytes().iter().for_each(|f| {
        letter_counts[(*f as usize) - 97] += 1;
    });

    for f in ransom_note.as_bytes().iter() {
        let val = &mut letter_counts[(*f as usize) - 97];
        if val == &0 {
            return false;
        }
        *val -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert!(can_construct("a".to_string(), "a".to_string()))
    }

    #[test]
    fn test_case_2() {
        assert!(!can_construct("aa".to_string(), "ba".to_string()))
    }

    #[test]
    fn test_case_3() {
        assert!(can_construct("aa".to_string(), "aab".to_string()))
    }

    #[test]
    fn test_case_4() {
        assert!(can_construct("xxyyzz".to_string(), "zzzyyyxxx".to_string()))
    }
}
