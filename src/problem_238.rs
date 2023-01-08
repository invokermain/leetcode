use std::{collections::VecDeque, iter::repeat};

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix = 1;
    let mut suffix = 1;
    let mut prefix_vec = vec![1];
    let mut suffix_vec = vec![1];

    for val in &nums[..nums.len() - 1] {
        prefix *= val;
        prefix_vec.push(prefix);
    }
    for val in nums[1..].iter().rev() {
        suffix *= val;
        suffix_vec.push(suffix);
    }

    Vec::from_iter((0..nums.len()).map(|idx| prefix_vec[idx] * suffix_vec[nums.len() - 1 - idx]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6])
    }

    #[test]
    fn test_case_2() {
        assert_eq!(product_except_self(vec![2, 3, 5, 4]), vec![60, 20, 24, 30])
    }
}

// Below are all my failed attempts which are correct but alas too slow. Although a
// non-O(n) solution managed to pass somehow (bottom one).

fn product_except_self_too_slow(nums: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::from_iter(repeat(1).take(nums.len()));

    for (idx, val) in nums.iter().enumerate() {
        for (idx_inner, val_inner) in output.iter_mut().enumerate() {
            if idx_inner != idx {
                *val_inner *= val;
            }
        }
    }

    output
}

fn product_except_self_too_slow_too(nums: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::with_capacity(nums.len());

    for idx in 0..nums.len() {
        output.push(
            nums.clone()
                .into_iter()
                .enumerate()
                .fold(
                    1,
                    |acc, (inner_idx, x)| if inner_idx == idx { acc } else { acc * x },
                ),
        )
    }

    output
}

fn product_except_self_also_too_slow(nums: Vec<i32>) -> Vec<i32> {
    Vec::from_iter((1..=nums.len()).map(|one_idx| {
        nums.iter()
            .cycle()
            .skip(one_idx)
            .take(nums.len() - 1)
            .fold(1, |acc: i32, x| acc * *x)
    }))
}

fn product_except_self_somehow_also_too_slow(nums: Vec<i32>) -> Vec<i32> {
    let mut stack = VecDeque::from_iter(nums.iter());
    let mut output = Vec::with_capacity(nums.len());

    for val in &nums {
        stack.pop_front();
        output.push(stack.iter().fold(1, |acc, x| acc * *x));
        stack.push_back(val);
    }

    output
}

fn product_except_self_somehow_passes_but_shouldnt(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix = 1;
    let mut output = Vec::with_capacity(nums.len());

    for (idx, val) in nums.iter().enumerate() {
        output.push(prefix * nums[idx + 1..nums.len()].iter().fold(1, |acc, x| acc * *x));
        prefix *= val;
    }

    output
}
