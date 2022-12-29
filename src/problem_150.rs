use std::collections::VecDeque;

fn apply_operator(operator: &str, left: i32, right: i32) -> i32 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => panic!("Unhandled match arm"),
    }
}

fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: VecDeque<i32> = VecDeque::new();

    for token in tokens {
        match token.as_str() {
            val @ ("+" | "-" | "*" | "/") => {
                let right = stack.pop_back().unwrap();
                let left = stack.pop_back().unwrap();
                stack.push_back(apply_operator(val, left, right));
            }
            val => stack.push_back(val.parse().unwrap()),
        }
    }

    stack[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec_of_string(vector: Vec<&str>) -> Vec<String> {
        vector.iter().map(|f| f.to_string()).collect()
    }

    #[test]
    fn test_case_1() {
        assert_eq!(eval_rpn(to_vec_of_string(vec!["1", "2", "+"])), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(eval_rpn(to_vec_of_string(vec!["1", "2", "-"])), -1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(eval_rpn(to_vec_of_string(vec!["2", "3", "*"])), 6);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(eval_rpn(to_vec_of_string(vec!["9", "3", "/"])), 3);
    }

    #[test]
    fn test_case_5() {
        assert_eq!(eval_rpn(to_vec_of_string(vec!["9"])), 9);
    }

    #[test]
    fn test_case_6() {
        assert_eq!(eval_rpn(to_vec_of_string(vec!["2", "1", "+", "3", "*"])), 9);
    }

    #[test]
    fn test_case_7() {
        assert_eq!(eval_rpn(to_vec_of_string(vec!["10","6","9","3","+","-11","*","/","*","17","+","5","+"])), 22);
    }
}
