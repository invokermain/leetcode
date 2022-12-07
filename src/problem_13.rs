// Roman Numeral UTF8 Decimal values
// I = 73
// V = 86
// X = 88
// L = 76
// C = 67
// D = 68
// M = 77

fn roman_to_int(s: String) -> i32 {
    // convert to bytes to make processing easier
    let bytes = s.as_bytes();

    let mut value = 0;
    let mut idx = 0;

    while idx < bytes.len() {
        let first = bytes[idx];
        let second = bytes.get(idx + 1).unwrap_or(&0);
        let mut double_match = true;

        value += match (first, second) {
            (73, 86) => 4,
            (73, 88) => 9,
            (88, 76) => 40,
            (88, 67) => 90,
            (67, 68) => 400,
            (67, 77) => 900,
            _ => {
                double_match = false;
                match first {
                    73 => 1,
                    86 => 5,
                    88 => 10,
                    76 => 50,
                    67 => 100,
                    68 => 500,
                    77 => 1000,
                    _ => panic!("Unrecognised Roman numeral"),
                }
            }
        };

        match double_match {
            true => idx += 2,
            false => idx += 1,
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(roman_to_int("I".into()), 1)
    }

    #[test]
    fn test_case_2() {
        assert_eq!(roman_to_int("II".into()), 2)
    }

    #[test]
    fn test_case_3() {
        assert_eq!(roman_to_int("III".into()), 3)
    }

    #[test]
    fn test_case_4() {
        assert_eq!(roman_to_int("IV".into()), 4)
    }

    #[test]
    fn test_case_5() {
        assert_eq!(roman_to_int("LVIII".into()), 58)
    }

    #[test]
    fn test_case_6() {
        assert_eq!(roman_to_int("MCMXCIV".into()), 1994)
    }
}
