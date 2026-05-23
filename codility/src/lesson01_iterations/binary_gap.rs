pub fn solution(n: i32) -> i32 {
    let bits = format!("{:b}", n);

    bits.trim_matches('0')
        .split('1')
        .map(|s| s.len() as i32)
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_gap() {
        assert_eq!(solution(9), 2);
        assert_eq!(solution(529), 4);
        assert_eq!(solution(20), 1);
        assert_eq!(solution(15), 0);
    }
}