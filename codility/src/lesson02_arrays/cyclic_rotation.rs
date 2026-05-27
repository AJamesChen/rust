/*
An array A consisting of N integers is given. Rotation of the array means that
each element is shifted right by one index, and the last element of the array is
moved to the first place.

For example, given:
    A = [3, 8, 9, 7, 6]
    K = 3

the function should return [9, 7, 6, 3, 8].
*/

pub fn solution(a: Vec<i32>, k: usize) -> Vec<i32> {
    let n = a.len();

    if n == 0 {
        return a;
    }

    let k = k % n;

    if k == 0 {
        return a;
    }

    let mut result = vec![0; n];

    for (i, value) in a.into_iter().enumerate() {
        let new_index = (i + k) % n;
        result[new_index] = value;
    }

    result
}

// Time Complexity: O(N), where N is the length of the input array.
// Space Complexity: O(N), for the rotated result array.

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_rotates(input: Vec<i32>, k: usize, expected: Vec<i32>) {
        assert_eq!(solution(input, k), expected);
    }

    #[test]
    fn test_array_38976() {
        assert_rotates(vec![3, 8, 9, 7, 6], 3, vec![9, 7, 6, 3, 8]);
    }

    #[test]
    fn test_array_000() {
        assert_rotates(vec![0, 0, 0], 1, vec![0, 0, 0]);
    }

    #[test]
    fn test_array_1234() {
        assert_rotates(vec![1, 2, 3, 4], 4, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_empty_array() {
        assert_rotates(Vec::new(), 2, Vec::new());
    }

    #[test]
    fn test_rotation_larger_than_length() {
        assert_rotates(vec![1, 2, 3, 4], 6, vec![3, 4, 1, 2]);
    }

    #[test]
    fn test_single_rotation_moves_last_item_to_front() {
        assert_rotates(vec![1, 2, 3, 4], 1, vec![4, 1, 2, 3]);
    }

    #[test]
    fn test_zero_rotations() {
        assert_rotates(vec![1, 2, 3, 4], 0, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_single_item_array() {
        assert_rotates(vec![7], 99, vec![7]);
    }

    #[test]
    fn test_negative_values() {
        assert_rotates(vec![-1, -2, -3, -4], 2, vec![-3, -4, -1, -2]);
    }
}
