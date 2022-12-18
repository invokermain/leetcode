fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut strength: Vec<(usize, i32)> = Vec::from_iter::<Vec<(usize, i32)>>(
        mat.iter()
            .enumerate()
            .map(|(idx, f)| (idx, f.iter().take_while(|v| **v == 1).count() as i32))
            .collect(),
    );
    strength.sort_by_key(|f| f.1);
    strength
        .iter()
        .take(k as usize)
        .map(|f| f.0 as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mat = vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(k_weakest_rows(mat, 3), vec![2, 0, 3])
    }

    #[test]
    fn test_case_2() {
        let mat = vec![
            vec![1, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        assert_eq!(k_weakest_rows(mat, 2), vec![0, 2])
    }
}
