/*
An array A consisting of N integers is given. All but one of the values occur
an even number of times.

Find the value that occurs an odd number of times.
*/

pub fn solution(a: Vec<i32>) -> i32 {
    // `fold` starts with 0 and XORs each value into the accumulator. Equal
    // values cancel each other out, leaving only the value with no pair.
    a.into_iter().fold(0, |unpaired, value| unpaired ^ value)
}

// Time Complexity: O(N), where N is the length of the input array.
// Space Complexity: O(1), using a single accumulator.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9393979() {
        assert_eq!(solution(vec![9, 3, 9, 3, 9, 7, 9]), 7);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(solution(vec![42]), 42);
    }

    #[test]
    fn test_unpaired_value_at_start() {
        assert_eq!(solution(vec![5, 1, 1, 2, 2]), 5);
    }

    #[test]
    fn test_unpaired_value_at_end() {
        assert_eq!(solution(vec![1, 1, 2, 2, 5]), 5);
    }

    #[test]
    fn test_multiple_repeated_pairs() {
        assert_eq!(solution(vec![4, 4, 4, 4, 6, 6, 8]), 8);
    }
}
