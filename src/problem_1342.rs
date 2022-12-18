const U32_BITS: i32 = (std::mem::size_of::<u32>() * 8) as i32;

fn number_of_steps(num: i32) -> i32 {
    let mut steps: i32 = 0;

    let mut val = num as u32;

    loop {
        if val == 0 {
            return steps;
        }

        if val.is_power_of_two() {
            return steps + U32_BITS - val.leading_zeros() as i32;
        }

        steps += 1;

        if val % 2 == 0 {
            val /= 2;
        } else {
            val -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(number_of_steps(14), 6)
    }

    #[test]
    fn test_case_2() {
        assert_eq!(number_of_steps(8), 4)
    }

    #[test]
    fn test_case_3() {
        assert_eq!(number_of_steps(123), 12)
    }

    #[test]
    fn test_case_4() {
        assert_eq!(number_of_steps(0), 0)
    }
}
