// You are given a string s and an integer k. You can choose any character of the string
// and change it to any other uppercase English character. You can perform this operation
// at most k times.
// Return the length of the longest substring containing the same letter you can get
// after performing the above operations.

// This algorithm uses a sliding window. At each point the size of our sliding window is
// constrained by the limit (k + "count of most popular character in substring").
// To track the "count of most popular character in substring" (`max_count`) we maintain
// an array of character counts.
// Therefore our algorithm tries to grow the window to the right at each step, if the
// above rule is violated we slide the window to the right (as we don't care about any
// windows that are smaller than our current maximum size). When we slide it to the right
// we decrease the character count of the character that is now outside the window.
fn character_replacement(s: String, k: i32) -> i32 {
    let s: &[u8] = s.as_bytes();

    let mut longest_substring = 0;
    let mut character_counts: [usize; 26] = [0; 26];
    let mut max_count = 0;
    let mut left = 0;

    for right in 0..s.len() {
        // translate the byte value of the char to a number in the range 0..26
        let right_char = (s[right] - b'A') as usize;

        // increment our new character count
        character_counts[right_char] += 1;

        // check if we have a new max value for "count of most popular character in substring"
        max_count = max_count.max(character_counts[right_char]);

        // our sliding window cannot grow anymore due to rule in description
        if right + 1 - left > k as usize + max_count {
            // decrease the character count of the character at the start of the window
            character_counts[(s[left] - b'A') as usize] -= 1;
            left += 1;
        }

        // check if we have a new longest_substring
        longest_substring = longest_substring.max(right + 1 - left)
    }

    longest_substring as i32
}

#[cfg(test)]
mod tests {
    use super::character_replacement;

    #[test]
    fn test_case_1() {
        assert_eq!(character_replacement("ABAB".into(), 2), 4)
    }

    #[test]
    fn test_case_2() {
        assert_eq!(character_replacement("AABABBA".into(), 1), 4)
    }

    #[test]
    fn test_case_3() {
        assert_eq!(character_replacement("AAAA".into(), 2), 4)
    }

    #[test]
    fn test_case_4() {
        assert_eq!(character_replacement("A".into(), 1), 1)
    }

    #[test]
    fn test_case_5() {
        assert_eq!(character_replacement("BAAAB".into(), 2), 5)
    }

    #[test]
    fn test_case_6() {
        assert_eq!(character_replacement("AAAB".into(), 0), 3)
    }
}
