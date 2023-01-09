// Given a string s, find the length of the longest substring without repeating characters.

use std::collections::VecDeque;

fn length_of_longest_substring(s: String) -> i32 {
    let mut longest_substring = 0;
    let mut current_sequence = VecDeque::new();

    for char in s.chars() {
        match current_sequence.contains(&char) {
            true => {
                if current_sequence.len() > longest_substring {
                    longest_substring = current_sequence.len();
                }
                while let Some(val) = current_sequence.pop_front() {
                    if val == char {
                        break;
                    }
                }
                current_sequence.push_back(char);
            }
            false => current_sequence.push_back(char),
        }
    }

    if current_sequence.len() > longest_substring {
        current_sequence.len() as i32
    } else {
        longest_substring as i32
    }
}

#[cfg(test)]
mod tests {
    use super::length_of_longest_substring;

    #[test]
    fn test_case_1() {
        assert_eq!(length_of_longest_substring("abcabcbb".into()), 3)
    }

    #[test]
    fn test_case_2() {
        assert_eq!(length_of_longest_substring("bbbbb".into()), 1)
    }

    #[test]
    fn test_case_3() {
        assert_eq!(length_of_longest_substring("pwwkew".into()), 3)
    }

    #[test]
    fn test_case_4() {
        assert_eq!(length_of_longest_substring("  []!".into()), 4)
    }

    #[test]
    fn test_case_5() {
        assert_eq!(length_of_longest_substring("".into()), 0)
    }

    #[test]
    fn test_case_6() {
        assert_eq!(length_of_longest_substring("aab".into()), 2)
    }

    #[test]
    fn test_case_7() {
        assert_eq!(length_of_longest_substring("dvdf".into()), 3)
    }
}
