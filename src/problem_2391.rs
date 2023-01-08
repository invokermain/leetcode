// DESCRIPTION
// You are given a 0-indexed array of strings garbage where garbage[i] represents the
// assortment of garbage at the ith house. garbage[i] consists only of the characters
// 'M', 'P' and 'G' representing one unit of metal, paper and glass garbage respectively.
// Picking up one unit of any type of garbage takes 1 minute.
// You are also given a 0-indexed integer array travel where travel[i] is the number of
// minutes needed to go from house i to house i + 1.
// There are three garbage trucks in the city, each responsible for picking up one type of
// garbage. Each garbage truck starts at house 0 and must visit each house in order;
// however, they do not need to visit every house.
// Only one garbage truck may be used at any given moment. While one truck is driving or
// picking up garbage, the other two trucks cannot do anything.
// Return the minimum number of minutes needed to pick up all the garbage.

fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    let mut minutes = [0, 0, 0]; // M, P, G
    let mut started_collection = [false, false, false]; // M, P ,G

    for (idx, garbage_str) in garbage.iter().rev().enumerate() {
        // calculate collection time
        for char in garbage_str.chars() {
            let idx = match char {
                'M' => 0,
                'P' => 1,
                'G' => 2,
                _ => panic!(),
            };
            started_collection[idx] = true;
            minutes[idx] += 1;
        }
        // add on travel time
        if idx < travel.len() {
            for i in 0..=2 {
                minutes[i] += started_collection[i] as i32 * travel[travel.len() - idx - 1]
            }
        }
    }

    minutes.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::garbage_collection;

    #[test]
    fn test_case_1() {
        assert_eq!(
            garbage_collection(
                vec!["G", "P", "GP", "GG"]
                    .into_iter()
                    .map(|x| x.into())
                    .collect(),
                vec![2, 4, 3]
            ),
            21
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            garbage_collection(
                vec!["MMM", "PGM", "GP"]
                    .into_iter()
                    .map(|x| x.into())
                    .collect(),
                vec![3, 10]
            ),
            37
        );
    }
}
