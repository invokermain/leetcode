const PERMUTATIONS: [[usize; 4]; 24] = [
    [0, 1, 2, 3],
    [0, 1, 3, 2],
    [0, 2, 1, 3],
    [0, 2, 3, 1],
    [0, 3, 1, 2],
    [0, 3, 2, 1],
    [1, 0, 2, 3],
    [1, 0, 3, 2],
    [1, 2, 0, 3],
    [1, 2, 3, 0],
    [1, 3, 0, 2],
    [1, 3, 2, 0],
    [2, 0, 1, 3],
    [2, 0, 3, 1],
    [2, 1, 0, 3],
    [2, 1, 3, 0],
    [2, 3, 0, 1],
    [2, 3, 1, 0],
    [3, 0, 1, 2],
    [3, 0, 2, 1],
    [3, 1, 0, 2],
    [3, 1, 2, 0],
    [3, 2, 0, 1],
    [3, 2, 1, 0],
];

fn largest_time_from_digits(arr: Vec<i32>) -> String {
    let mut solutions = PERMUTATIONS
        .iter()
        .map(|perm| {
            (
                arr[perm[0]] * 10 + arr[perm[1]],
                arr[perm[2]] * 10 + arr[perm[3]],
            )
        })
        .filter(|&sol| sol.0 <= 23 && sol.1 <= 59)
        .collect::<Vec<(i32, i32)>>();

    solutions.sort();

    match solutions.last() {
        None => "".to_string(),
        Some(solution) => format!("{:02}:{:02}", solution.0, solution.1),
    }
}

mod tests {
    use crate::problem_949::largest_time_from_digits;

    #[test]
    fn test_case_1() {
        let arr = vec![1i32, 2, 3, 4];
        assert_eq!(largest_time_from_digits(arr), "23:41".to_string())
    }

    #[test]
    fn test_case_2() {
        let arr = vec![5i32, 5, 5, 5];
        assert_eq!(largest_time_from_digits(arr), "".to_string())
    }

    #[test]
    fn test_case_3() {
        let arr = vec![0i32, 0, 0, 0];
        assert_eq!(largest_time_from_digits(arr), "00:00".to_string())
    }

    #[test]
    fn test_case_4() {
        let arr = vec![1i32, 1, 5, 9];
        assert_eq!(largest_time_from_digits(arr), "19:51".to_string())
    }

    #[test]
    fn test_case_5() {
        let arr = vec![0i32, 0, 1, 0];
        assert_eq!(largest_time_from_digits(arr), "10:00".to_string())
    }

    #[test]
    fn test_case_6() {
        let arr = vec![0i32, 0, 3, 1];
        assert_eq!(largest_time_from_digits(arr), "13:00".to_string())
    }

    #[test]
    fn test_case_7() {
        let arr = vec![2i32, 0, 6, 6];
        assert_eq!(largest_time_from_digits(arr), "06:26".to_string())
    }
}
