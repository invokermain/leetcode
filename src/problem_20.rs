use std::collections::VecDeque;

#[inline]
fn reflect(left: char) -> char {
    match left {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        _ => panic!("Unhandled match arm")
    }
}

fn is_valid(s: String) -> bool {
    let mut stack: VecDeque<char> = VecDeque::new();

    for item in s.chars() {
        match item {
            val  @ (')' | '}' | ']') => if stack.pop_front().unwrap_or('x') != reflect(val) { return false; },
            val =>  stack.push_front(val)
        };
    }

    return stack.is_empty();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert!(is_valid(String::from("()")));
    }

    #[test]
    fn test_case_2() {
        assert!(is_valid(String::from("()[]{}")));
    }

    #[test]
    fn test_case_3() {
        assert!(!is_valid(String::from("(]")));
    }

    #[test]
    fn test_case_4() {
        assert!(!is_valid(String::from("(")));
    }
}