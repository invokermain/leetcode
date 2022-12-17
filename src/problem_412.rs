const FIZZ: &str = "Fizz";
const BUZZ: &str = "Buzz";
const FIZZBUZZ: &str = "FizzBuzz";

fn fizz_buzz(n: i32) -> Vec<String> {
    Vec::from_iter((1..=n).map(|f| {
        let div_by_3 = f % 3 == 0;
        let div_by_5 = f % 5 == 0;
        if div_by_3 && div_by_5 {
            FIZZBUZZ.to_string()
        } else if div_by_3 {
            FIZZ.to_string()
        } else if div_by_5 {
            BUZZ.to_string()
        } else {
            f.to_string()
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            fizz_buzz(3),
            vec![String::from("1"), String::from("2"), String::from("Fizz")]
        )
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            fizz_buzz(5),
            vec![
                String::from("1"),
                String::from("2"),
                String::from("Fizz"),
                String::from("4"),
                String::from("Buzz")
            ]
        )
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            fizz_buzz(15),
            vec![
                String::from("1"),
                String::from("2"),
                String::from("Fizz"),
                String::from("4"),
                String::from("Buzz"),
                String::from("Fizz"),
                String::from("7"),
                String::from("8"),
                String::from("Fizz"),
                String::from("Buzz"),
                String::from("11"),
                String::from("Fizz"),
                String::from("13"),
                String::from("14"),
                String::from("FizzBuzz"),
            ]
        )
    }
}
